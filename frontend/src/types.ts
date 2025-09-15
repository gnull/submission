export interface Problem {
  id: number;
  name: string;
  desc: string;
}

export interface CreateProblem {
  name: string;
  desc: string;
}

export interface FileInfo {
  id: number;
  name: string;
  hash: string;
}

export interface Feedback {
  id: number;
  grade: number;
  message?: string;
}

export interface SubmissionStatus {
  accepted: boolean;
  feedbacks: Feedback[];
}

export interface Submission {
  id: number;
  comment: string;
  problem: number;
  files: FileInfo[];
  status: SubmissionStatus;
}

export interface CreateSubmission {
  comment: string;
}

export interface CreateFeedback {
  grade: number;
  message?: string;
}

export interface ProblemWithStats {
  id: number;
  name: string;
  desc: string;
  attempts: number;
  accepted: boolean;
}

export type UserRole = "teacher" | "student";
