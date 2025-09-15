import axios from "axios";
import type {
  Problem,
  CreateProblem,
  Submission,
  CreateFeedback,
  ProblemWithStats,
} from "./types";

const api = axios.create({
  baseURL: "/api",
  headers: {
    "Content-Type": "application/json",
  },
});

export class ApiService {
  // Problem endpoints
  async getProblems(): Promise<Problem[]> {
    const response = await api.get("/problems");
    return response.data;
  }

  async getProblem(id: number): Promise<Problem> {
    const response = await api.get(`/problems/${id}`);
    return response.data;
  }

  async createProblem(problem: CreateProblem): Promise<{ id: number }> {
    const response = await api.post("/problems", problem);
    return response.data;
  }

  // Submission endpoints
  async getSubmissions(): Promise<Submission[]> {
    const response = await api.get("/submissions");
    return response.data;
  }

  async getSubmission(id: number): Promise<Submission> {
    const response = await api.get(`/submissions/${id}`);
    return response.data;
  }

  async createSubmission(
    problemId: number,
    comment: string,
    files: File[],
  ): Promise<{ id: number }> {
    const formData = new FormData();
    formData.append("comment", comment);

    files.forEach((file, index) => {
      formData.append(`file${index}`, file);
    });

    const response = await api.post(
      `/problems/${problemId}/submissions`,
      formData,
      {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      },
    );
    return response.data;
  }

  // Feedback endpoints
  async createFeedback(
    submissionId: number,
    feedback: CreateFeedback,
  ): Promise<{ id: number }> {
    const response = await api.post(
      `/submissions/${submissionId}/feedback`,
      feedback,
    );
    return response.data;
  }

  // File download
  async downloadFile(hash: string): Promise<Blob> {
    const response = await api.get(`/files/${hash}`, {
      responseType: "blob",
    });
    return response.data;
  }

  // Enhanced methods for the frontend
  async getProblemsWithStats(): Promise<ProblemWithStats[]> {
    const [problems, submissions] = await Promise.all([
      this.getProblems(),
      this.getSubmissions(),
    ]);

    return problems.map((problem) => {
      const problemSubmissions = submissions.filter(
        (s) => s.problem === problem.id,
      );
      const attempts = problemSubmissions.length;
      const accepted = problemSubmissions.some((s) => s.status.accepted);

      return {
        ...problem,
        attempts,
        accepted,
      };
    });
  }

  async getSubmissionsForProblem(problemId: number): Promise<Submission[]> {
    const submissions = await this.getSubmissions();
    return submissions.filter((s) => s.problem === problemId);
  }
}

export const apiService = new ApiService();
