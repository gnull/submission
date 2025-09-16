use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::Path;
use turso::{Builder, Connection};

pub struct SubmDb {
    conn: Connection,
    uploads_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    pub id: i64,
    pub name: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Submission {
    pub id: i64,
    pub comment: String,
    pub problem: i64,
    pub files: Vec<FileInfo>,
    pub status: SubmissionStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub id: i64,
    pub name: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feedback {
    pub id: i64,
    pub grade: i64,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionStatus {
    pub accepted: bool,
    pub feedbacks: Vec<Feedback>,
}

#[derive(Deserialize)]
pub struct CreateProblem {
    pub name: String,
    pub desc: String,
}

#[derive(Deserialize)]
pub struct CreateSubmission {
    pub comment: String,
}

#[derive(Deserialize)]
pub struct CreateFeedback {
    pub grade: i64,
    pub message: Option<String>,
}

impl SubmDb {
    pub async fn new(s: &str, uploads_dir: &str) -> Self {
        let db = Builder::new_local(s).build().await.unwrap();

        // Create uploads directory if it doesn't exist
        if !Path::new(uploads_dir).exists() {
            fs::create_dir_all(uploads_dir).expect("Failed to create uploads directory");
        }

        SubmDb {
            conn: db.connect().unwrap(),
            uploads_dir: uploads_dir.to_string(),
        }
    }

    pub async fn init(&mut self) {
        self.conn
            .execute_batch(
                "
              CREATE TABLE IF NOT EXISTS problem (
                  id INTEGER PRIMARY KEY,
                  name TEXT,
                  desc TEXT
              );
              CREATE TABLE IF NOT EXISTS file (
                  id INTEGER PRIMARY KEY,
                  name TEXT,
                  hash TEXT
              );
              CREATE TABLE IF NOT EXISTS submission (
                  id INTEGER PRIMARY KEY,
                  comment TEXT,
                  problem INTEGER,
                  FOREIGN KEY (problem) REFERENCES problem(id)
              );
              CREATE TABLE IF NOT EXISTS attachment (
                  submission INTEGER,
                  file INTEGER,
                  FOREIGN KEY (submission) REFERENCES submission(id),
                  FOREIGN KEY (file) REFERENCES file(id)
              );
              CREATE TABLE IF NOT EXISTS feedback (
                  id INTEGER PRIMARY KEY,
                  submission INTEGER,
                  grade INTEGER CHECK (grade IN (0, 1)),
                  message TEXT,
                  FOREIGN KEY (submission) REFERENCES submission(id)
              );
            ",
            )
            .await
            .unwrap();
    }

    pub async fn create_problem(
        &mut self,
        problem: CreateProblem,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        self.conn
            .execute(
                "INSERT INTO problem (name, desc) VALUES (?, ?)",
                [problem.name.as_str(), problem.desc.as_str()],
            )
            .await?;

        let id = self.conn.last_insert_rowid();
        Ok(id)
    }

    pub async fn get_problems(&mut self) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
        let mut rows = self
            .conn
            .query("SELECT id, name, desc FROM problem ORDER BY id", ())
            .await?;

        let mut problems = Vec::new();
        while let Some(row) = rows.next().await? {
            problems.push(Problem {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
            });
        }
        Ok(problems)
    }

    pub async fn get_problem_by_id(
        &mut self,
        id: i64,
    ) -> Result<Option<Problem>, Box<dyn std::error::Error>> {
        let mut rows = self
            .conn
            .query("SELECT id, name, desc FROM problem WHERE id = ?", [id])
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(Problem {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn store_file(
        &mut self,
        filename: &str,
        content: &[u8],
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Calculate SHA256 hash
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());

        // Store file with hash as filename
        let file_path = format!("{}/{}", self.uploads_dir, hash);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content)?;

        // Insert file info into database (check if exists first)
        let mut existing_rows = self
            .conn
            .query("SELECT id FROM file WHERE hash = ?", [hash.as_str()])
            .await?;

        if existing_rows.next().await?.is_none() {
            self.conn
                .execute(
                    "INSERT INTO file (name, hash) VALUES (?, ?)",
                    [filename, hash.as_str()],
                )
                .await?;
        }

        Ok(hash)
    }

    pub async fn create_submission_with_files(
        &mut self,
        problem_id: i64,
        submission: CreateSubmission,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Create submission
        self.conn
            .execute(
                "INSERT INTO submission (comment, problem) VALUES (?, ?)",
                [submission.comment.as_str(), problem_id.to_string().as_str()],
            )
            .await?;

        let submission_id = self.conn.last_insert_rowid();

        // Store files and create attachments
        for (filename, content) in files {
            let hash = self.store_file(&filename, &content).await?;

            // Get file ID
            let mut file_rows = self
                .conn
                .query("SELECT id FROM file WHERE hash = ?", [hash.as_str()])
                .await?;
            let file_row = file_rows.next().await?.ok_or("File not found")?;
            let file_id: i64 = file_row.get(0)?;

            // Create attachment
            self.conn
                .execute(
                    "INSERT INTO attachment (submission, file) VALUES (?, ?)",
                    [
                        submission_id.to_string().as_str(),
                        file_id.to_string().as_str(),
                    ],
                )
                .await?;
        }

        Ok(submission_id)
    }

    pub async fn get_submissions(&mut self) -> Result<Vec<Submission>, Box<dyn std::error::Error>> {
        let mut rows = self
            .conn
            .query(
                "SELECT id, comment, problem FROM submission ORDER BY id DESC",
                (),
            )
            .await?;

        let mut submissions = Vec::new();
        while let Some(row) = rows.next().await? {
            let submission_id: i64 = row.get(0)?;
            let comment: String = row.get(1)?;
            let problem: i64 = row.get(2)?;

            // Get files for this submission
            let files = self.get_submission_files(submission_id).await?;

            // Get feedback for this submission
            let feedbacks = self.get_submission_feedbacks(submission_id).await?;
            let accepted = feedbacks.last().map_or(false, |f| f.grade == 1);

            submissions.push(Submission {
                id: submission_id,
                comment,
                problem,
                files,
                status: SubmissionStatus {
                    accepted,
                    feedbacks,
                },
            });
        }
        Ok(submissions)
    }

    pub async fn get_submission_by_id(
        &mut self,
        id: i64,
    ) -> Result<Option<Submission>, Box<dyn std::error::Error>> {
        let mut rows = self
            .conn
            .query(
                "SELECT id, comment, problem FROM submission WHERE id = ?",
                [id],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            let submission_id: i64 = row.get(0)?;
            let comment: String = row.get(1)?;
            let problem: i64 = row.get(2)?;

            // Get files for this submission
            let files = self.get_submission_files(submission_id).await?;

            // Get feedback for this submission
            let feedbacks = self.get_submission_feedbacks(submission_id).await?;
            let accepted = feedbacks.last().map_or(false, |f| f.grade == 1);

            Ok(Some(Submission {
                id: submission_id,
                comment,
                problem,
                files,
                status: SubmissionStatus {
                    accepted,
                    feedbacks,
                },
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_submission_files(
        &mut self,
        submission_id: i64,
    ) -> Result<Vec<FileInfo>, Box<dyn std::error::Error>> {
        let mut rows = self.conn.query(
            "SELECT f.id, f.name, f.hash FROM file f JOIN attachment a ON f.id = a.file WHERE a.submission = ?",
            [submission_id.to_string().as_str()]
        ).await?;

        let mut files = Vec::new();
        while let Some(row) = rows.next().await? {
            files.push(FileInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                hash: row.get(2)?,
            });
        }
        Ok(files)
    }

    async fn get_submission_feedbacks(
        &mut self,
        submission_id: i64,
    ) -> Result<Vec<Feedback>, Box<dyn std::error::Error>> {
        let mut rows = self
            .conn
            .query(
                "SELECT id, grade, message FROM feedback WHERE submission = ? ORDER BY id",
                [submission_id.to_string().as_str()],
            )
            .await?;

        let mut feedbacks = Vec::new();
        while let Some(row) = rows.next().await? {
            feedbacks.push(Feedback {
                id: row.get(0)?,
                grade: row.get(1)?,
                message: row.get(2)?,
            });
        }
        Ok(feedbacks)
    }

    pub async fn create_feedback(
        &mut self,
        submission_id: i64,
        feedback: CreateFeedback,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Validate grade is 0 or 1
        if feedback.grade != 0 && feedback.grade != 1 {
            return Err("Grade must be either 0 or 1".into());
        }

        // Always insert new feedback (multiple feedbacks allowed)
        self.conn
            .execute(
                "INSERT INTO feedback (submission, grade, message) VALUES (?, ?, ?)",
                [
                    submission_id.to_string().as_str(),
                    feedback.grade.to_string().as_str(),
                    feedback.message.as_deref().unwrap_or(""),
                ],
            )
            .await?;

        let feedback_id = self.conn.last_insert_rowid();
        Ok(feedback_id)
    }

    pub async fn get_file_content(
        &self,
        hash: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}", self.uploads_dir, hash);
        let content = fs::read(&file_path)?;
        Ok(content)
    }
}
