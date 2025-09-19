<template>
    <div class="student-view">
        <!-- Header Section -->
        <div class="header">
            <h1>Панель ученика</h1>
            <div class="stats">
                <StatCard :number="acceptedProblems" label="Принято" />
                <StatCard :number="totalAttempts" label="Всего попыток" />
            </div>
        </div>

        <!-- Success/Error Messages -->
        <Message
            v-if="message"
            :severity="messageType"
            :closable="true"
            @close="message = ''"
        >
            {{ message }}
        </Message>

        <!-- Loading State -->
        <div v-if="loading" class="loading-container">
            <ProgressSpinner />
            <p>Загружаем задачи...</p>
        </div>

        <!-- Problems List -->
        <div v-if="!loading" class="problems-section">
            <h2>Доступные задачи</h2>
            <div v-if="problemsWithStats.length === 0" class="empty-state">
                <Message severity="info" :closable="false">
                    Пока нет доступных задач. Заходите позже!
                </Message>
            </div>

            <div v-else class="problems-grid">
                <Card
                    v-for="problem in problemsWithStats"
                    :key="problem.id"
                    :class="{ 'problem-accepted': problem.accepted }"
                    class="problem-card"
                >
                    <template #header>
                        <div class="problem-header">
                            <h3>{{ problem.name }}</h3>
                            <div class="problem-badges">
                                <Tag
                                    v-if="problem.accepted"
                                    severity="success"
                                    icon="pi pi-check"
                                    value="Принято"
                                />
                                <Tag
                                    severity="secondary"
                                    :value="`${problem.attempts} попыток`"
                                />
                            </div>
                        </div>
                    </template>
                    <template #content>
                        <p class="problem-description">
                            {{ truncateText(problem.desc, 150) }}
                        </p>
                    </template>
                    <template #footer>
                        <div class="problem-actions">
                            <Button
                                :label="
                                    problem.attempts > 0
                                        ? 'Открыть и переотправить'
                                        : 'Открыть задачу'
                                "
                                @click="openProblemDialog(problem)"
                                icon="pi pi-external-link"
                                class="p-button-primary"
                            />
                            <Button
                                v-if="problem.attempts > 0"
                                label="Мои работы"
                                @click="viewSubmissions(problem.id)"
                                icon="pi pi-list"
                                severity="secondary"
                            />
                        </div>
                    </template>
                </Card>
            </div>
        </div>

        <!-- Problem Details and Submission Dialog -->
        <Dialog
            v-model:visible="showProblemDialog"
            :header="`${selectedProblem?.name || 'Задача'}`"
            modal
            :style="{ width: '90vw', maxWidth: '900px' }"
            :maximizable="true"
        >
            <div v-if="selectedProblem" class="problem-dialog-content">
                <!-- Problem Description -->
                <div class="problem-description-section">
                    <h3>Описание задачи</h3>
                    <div class="problem-description-text">
                        {{ selectedProblem.desc }}
                    </div>
                </div>

                <!-- Submission Form -->
                <div class="submission-form-section">
                    <h3>Отправить решение</h3>

                    <!-- Validation Error -->
                    <Message
                        v-if="submissionValidationError"
                        severity="error"
                        :closable="true"
                        @close="submissionValidationError = ''"
                    >
                        {{ submissionValidationError }}
                    </Message>

                    <!-- Success Message -->
                    <Message
                        v-if="submissionSuccessMessage"
                        severity="success"
                        :closable="true"
                        @close="submissionSuccessMessage = ''"
                    >
                        {{ submissionSuccessMessage }}
                    </Message>

                    <form
                        @submit.prevent="submitSolution"
                        class="submission-form"
                    >
                        <div class="form-group">
                            <label class="form-label"
                                >Комментарий (необязательно)</label
                            >
                            <textarea
                                v-model="submissionComment"
                                class="form-control"
                                rows="3"
                                placeholder="Добавьте комментарии к вашему решению..."
                            ></textarea>
                        </div>

                        <div class="form-group">
                            <label class="form-label">Загрузить файлы *</label>
                            <div class="file-upload">
                                <input
                                    ref="fileInput"
                                    type="file"
                                    multiple
                                    @change="handleFileSelect"
                                    class="file-input"
                                    accept=".py,.js,.java,.cpp,.c,.h,.txt,.md"
                                />
                                <div
                                    class="file-upload-label"
                                    @click="fileInput?.click()"
                                >
                                    <span
                                        v-if="
                                            selectedSubmissionFiles.length === 0
                                        "
                                    >
                                        📁 Выберите файлы для загрузки
                                    </span>
                                    <span v-else>
                                        Выбрано файлов:
                                        {{ selectedSubmissionFiles.length }}
                                    </span>
                                </div>
                            </div>
                            <div class="help-text">
                                Пожалуйста, выберите хотя бы один файл для
                                отправки решения.
                            </div>
                            <div
                                v-if="selectedSubmissionFiles.length > 0"
                                class="selected-files"
                            >
                                <div
                                    v-for="(
                                        file, index
                                    ) in selectedSubmissionFiles"
                                    :key="index"
                                    class="file-item-upload"
                                >
                                    <span class="file-name">{{
                                        file.name
                                    }}</span>
                                    <button
                                        type="button"
                                        @click="removeSubmissionFile(index)"
                                        class="file-remove"
                                    >
                                        ×
                                    </button>
                                </div>
                            </div>
                        </div>

                        <div class="form-actions">
                            <Button
                                type="submit"
                                :label="
                                    submittingSolution
                                        ? 'Отправляем...'
                                        : selectedSubmissionFiles.length === 0
                                          ? 'Выберите файлы для отправки'
                                          : 'Отправить решение'
                                "
                                :disabled="
                                    submittingSolution ||
                                    selectedSubmissionFiles.length === 0
                                "
                                severity="success"
                            />
                        </div>
                    </form>
                </div>
            </div>
        </Dialog>

        <!-- My Submissions Dialog -->
        <Dialog
            v-model:visible="showSubmissions"
            :header="`Мои работы по задаче &quot;${selectedProblemName}&quot;`"
            modal
            :style="{ width: '90vw', maxWidth: '800px' }"
            :maximizable="true"
        >
            <div v-if="selectedSubmissions.length === 0" class="empty-state">
                <Message severity="info" :closable="false">
                    Работы не найдены.
                </Message>
            </div>
            <div v-else class="submissions-list">
                <Card
                    v-for="submission in selectedSubmissions"
                    :key="submission.id"
                    class="submission-card"
                >
                    <template #header>
                        <div class="submission-header">
                            <h4>Работа #{{ submission.id }}</h4>
                            <Tag
                                :severity="
                                    submission.status.accepted
                                        ? 'success'
                                        : 'danger'
                                "
                                :value="
                                    submission.status.accepted
                                        ? 'Принято'
                                        : 'Не принято'
                                "
                                :icon="
                                    submission.status.accepted
                                        ? 'pi pi-check'
                                        : 'pi pi-times'
                                "
                            />
                        </div>
                    </template>
                    <template #content>
                        <div class="submission-details">
                            <div class="submission-comment">
                                <strong>Комментарий:</strong>
                                {{
                                    submission.comment ||
                                    "Комментарий не добавлен"
                                }}
                            </div>

                            <div class="submission-files">
                                <strong>Отправленные файлы:</strong>
                                <div
                                    v-if="submission.files.length === 0"
                                    class="no-files"
                                >
                                    <Message severity="info" :closable="false">
                                        Файлы не были отправлены.
                                    </Message>
                                </div>
                                <div v-else class="files-list">
                                    <div
                                        v-for="file in submission.files"
                                        :key="file.id"
                                        class="file-item"
                                    >
                                        <div class="file-info">
                                            <span class="file-name"
                                                >📄 {{ file.name }}</span
                                            >
                                            <div class="file-actions">
                                                <Button
                                                    label="📥 Скачать"
                                                    @click="downloadFile(file)"
                                                    size="small"
                                                    severity="secondary"
                                                />
                                                <Button
                                                    :label="
                                                        previewingFile ===
                                                        file.id
                                                            ? '👁️ Скрыть'
                                                            : '👁️ Просмотр'
                                                    "
                                                    @click="togglePreview(file)"
                                                    size="small"
                                                    severity="secondary"
                                                />
                                            </div>
                                        </div>

                                        <!-- File Preview -->
                                        <div
                                            v-if="previewingFile === file.id"
                                            class="file-preview"
                                        >
                                            <div class="preview-header">
                                                <h5>
                                                    Просмотр: {{ file.name }}
                                                </h5>
                                            </div>
                                            <div class="preview-content">
                                                <div
                                                    v-if="loadingPreview"
                                                    class="loading-preview"
                                                >
                                                    <ProgressSpinner />
                                                    <p>Загружаем просмотр...</p>
                                                </div>
                                                <div
                                                    v-else-if="previewError"
                                                    class="preview-error"
                                                >
                                                    <Message
                                                        severity="error"
                                                        :closable="false"
                                                    >
                                                        Не удалось загрузить
                                                        просмотр. Попробуйте
                                                        скачать файл.
                                                    </Message>
                                                </div>
                                                <pre
                                                    v-else
                                                    class="code-preview"
                                                    >{{
                                                        filePreviewContent
                                                    }}</pre
                                                >
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="feedback-section">
                                <div
                                    class="feedback-container"
                                    v-if="
                                        submission.status.feedbacks.length > 0
                                    "
                                >
                                    <strong>Обратная связь:</strong>
                                    <div class="feedback-list">
                                        <Card
                                            v-for="feedback in submission.status
                                                .feedbacks"
                                            :key="feedback.id"
                                            :class="
                                                feedback.grade === 1
                                                    ? 'feedback-accept'
                                                    : 'feedback-reject'
                                            "
                                            class="feedback-item"
                                        >
                                            <template #content>
                                                <div class="feedback-content">
                                                    <Tag
                                                        :severity="
                                                            feedback.grade === 1
                                                                ? 'success'
                                                                : 'danger'
                                                        "
                                                        :value="
                                                            feedback.grade === 1
                                                                ? 'Принято'
                                                                : 'Отклонено'
                                                        "
                                                        :icon="
                                                            feedback.grade === 1
                                                                ? 'pi pi-check'
                                                                : 'pi pi-times'
                                                        "
                                                    />
                                                    <div
                                                        v-if="feedback.message"
                                                        class="feedback-message"
                                                    >
                                                        {{ feedback.message }}
                                                    </div>
                                                </div>
                                            </template>
                                        </Card>
                                    </div>
                                </div>
                                <div v-else class="no-feedback">
                                    <Message severity="warn" :closable="false">
                                        Обратной связи пока нет - ждём проверки
                                    </Message>
                                </div>
                            </div>
                        </div>
                    </template>
                </Card>
            </div>
        </Dialog>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { apiService } from "../api";
import type { ProblemWithStats, Submission, Problem, FileInfo } from "../types";

// Reactive state
const problemsWithStats = ref<ProblemWithStats[]>([]);
const allSubmissions = ref<Submission[]>([]);
const selectedSubmissions = ref<Submission[]>([]);
const selectedProblemName = ref("");
const loading = ref(true);
const showSubmissions = ref(false);
const message = ref("");
const messageType = ref<"success" | "error" | "info" | "warn">("success");
const previewingFile = ref<number | null>(null);
const filePreviewContent = ref("");
const loadingPreview = ref(false);
const previewError = ref(false);

// Problem dialog and submission form state
const showProblemDialog = ref(false);
const selectedProblem = ref<Problem | null>(null);
const submissionComment = ref("");
const selectedSubmissionFiles = ref<File[]>([]);
const submittingSolution = ref(false);
const submissionValidationError = ref("");
const submissionSuccessMessage = ref("");
const fileInput = ref<HTMLInputElement>();

// Computed properties
const acceptedProblems = computed(
    () => problemsWithStats.value.filter((p) => p.accepted).length,
);

const totalAttempts = computed(() =>
    problemsWithStats.value.reduce((sum, p) => sum + p.attempts, 0),
);

// Methods
const loadData = async () => {
    try {
        loading.value = true;
        const [problems, submissions] = await Promise.all([
            apiService.getProblems(),
            apiService.getSubmissions(),
        ]);

        allSubmissions.value = submissions;

        // Calculate stats for each problem
        problemsWithStats.value = problems.map((problem) => {
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
    } catch (error) {
        showMessage("Не удалось загрузить задачи", "error");
        console.error("Error loading data:", error);
    } finally {
        loading.value = false;
    }
};

const viewSubmissions = (problemId: number) => {
    const problem = problemsWithStats.value.find((p) => p.id === problemId);
    if (!problem) return;

    selectedProblemName.value = problem.name;
    selectedSubmissions.value = allSubmissions.value.filter(
        (s) => s.problem === problemId,
    );
    showSubmissions.value = true;
};

const truncateText = (text: string, length: number): string => {
    if (text.length <= length) return text;
    return text.substring(0, length) + "...";
};

const downloadFile = async (file: FileInfo) => {
    try {
        const blob = await apiService.downloadFile(file.hash);
        const url = window.URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = file.name;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        window.URL.revokeObjectURL(url);
    } catch (err) {
        console.error("Error downloading file:", err);
        showMessage("Не удалось скачать файл", "error");
    }
};

const togglePreview = async (file: FileInfo) => {
    if (previewingFile.value === file.id) {
        // If this file is already being previewed, close it
        closePreview();
    } else {
        // If this file is not being previewed, preview it
        await previewFile(file);
    }
};

const previewFile = async (file: FileInfo) => {
    try {
        previewingFile.value = file.id;
        loadingPreview.value = true;
        previewError.value = false;

        const blob = await apiService.downloadFile(file.hash);
        const text = await blob.text();
        filePreviewContent.value = text;
    } catch (err) {
        console.error("Error previewing file:", err);
        previewError.value = true;
    } finally {
        loadingPreview.value = false;
    }
};

const closePreview = () => {
    previewingFile.value = null;
    filePreviewContent.value = "";
    previewError.value = false;
};

const showMessage = (
    msg: string,
    type: "success" | "error" | "info" | "warn",
) => {
    message.value = msg;
    messageType.value = type;
    setTimeout(() => {
        message.value = "";
    }, 5000);
};

const openProblemDialog = (problem: ProblemWithStats) => {
    selectedProblem.value = problem as Problem;
    showProblemDialog.value = true;
    // Reset form state when opening dialog
    submissionComment.value = "";
    selectedSubmissionFiles.value = [];
    submissionValidationError.value = "";
    submissionSuccessMessage.value = "";
    if (fileInput.value) {
        fileInput.value.value = "";
    }
};

const handleFileSelect = (event: Event) => {
    const target = event.target as HTMLInputElement;
    if (target.files) {
        selectedSubmissionFiles.value = Array.from(target.files);
        // Clear validation error when files are selected
        if (selectedSubmissionFiles.value.length > 0) {
            submissionValidationError.value = "";
        }
    }
};

const removeSubmissionFile = (index: number) => {
    selectedSubmissionFiles.value.splice(index, 1);
};

const submitSolution = async () => {
    // Clear any previous validation errors
    submissionValidationError.value = "";
    submissionSuccessMessage.value = "";

    if (selectedSubmissionFiles.value.length === 0) {
        submissionValidationError.value =
            "⚠️ Пожалуйста, выберите хотя бы один файл для отправки решения";
        // Clear validation error after 8 seconds
        setTimeout(() => {
            submissionValidationError.value = "";
        }, 8000);
        return;
    }

    if (!selectedProblem.value) {
        submissionValidationError.value = "Ошибка: задача не выбрана";
        return;
    }

    try {
        submittingSolution.value = true;
        await apiService.createSubmission(
            selectedProblem.value.id,
            submissionComment.value,
            selectedSubmissionFiles.value,
        );

        submissionSuccessMessage.value = "Решение успешно отправлено!";
        submissionComment.value = "";
        selectedSubmissionFiles.value = [];
        if (fileInput.value) {
            fileInput.value.value = "";
        }

        // Reload data to update stats
        await loadData();

        setTimeout(() => {
            submissionSuccessMessage.value = "";
            showProblemDialog.value = false;
        }, 3000);
    } catch (err) {
        submissionValidationError.value =
            "Не удалось отправить решение. Попробуйте ещё раз.";
        // Clear validation error after 5 seconds
        setTimeout(() => {
            submissionValidationError.value = "";
        }, 5000);
        console.error("Error submitting solution:", err);
    } finally {
        submittingSolution.value = false;
    }
};

// Lifecycle
onMounted(() => {
    loadData();
});
</script>

<style scoped>
.student-view {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid var(--p-surface-border);
}

.header h1 {
    color: var(--p-text-color);
    margin: 0;
    font-size: 2rem;
}

.stats {
    display: flex;
    gap: 1rem;
}

.loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 3rem;
    gap: 1rem;
}

.problems-section h2 {
    margin-bottom: 1.5rem;
    color: var(--p-text-color);
}

.problems-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1.5rem;
    width: 100%;
}

.problem-card {
    transition:
        transform 0.2s,
        box-shadow 0.2s;
}

.problem-card:hover {
    transform: translateY(-2px);
}

.problem-accepted {
    border-left: 4px solid var(--p-green-500);
    background: linear-gradient(135deg, #f0f9f0 0%, #e8f5e8 100%);
    box-shadow: 0 2px 8px rgba(34, 197, 94, 0.15);
}

.problem-card:not(.problem-accepted) {
    border-left: 4px solid #64748b;
}

.problem-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    padding: 1rem;
}

.problem-header h3 {
    margin: 0;
    color: var(--p-text-color);
    flex: 1;
}

.problem-accepted .problem-header h3 {
    color: #16a34a;
    font-weight: 600;
}

.problem-badges {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-end;
}

.problem-description {
    color: var(--p-text-muted-color);
    line-height: 1.5;
    margin-bottom: 0;
}

.problem-accepted .problem-description {
    color: #15803d;
}

.problem-accepted .problem-badges .p-tag {
    background: #ffffff !important;
    color: #374151 !important;
    border: 1px solid #d1d5db !important;
}

.problem-accepted .problem-actions .p-button-secondary {
    background: #ffffff !important;
    border: 1px solid #d1d5db !important;
    color: #374151 !important;
}

.problem-accepted .problem-actions .p-button-secondary:hover {
    background: #f9fafb !important;
    border: 1px solid #9ca3af !important;
    color: #1f2937 !important;
}

.problem-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
}

.empty-state {
    text-align: center;
    padding: 2rem;
}

.submissions-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.submission-card {
    border: 1px solid var(--p-surface-border);
    border-radius: var(--p-border-radius);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
}

.submission-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    gap: 1rem;
}

.submission-header h4 {
    margin: 0;
    color: var(--p-text-color);
    flex: 1;
}

.submission-details {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.submission-comment,
.submission-files {
    color: var(--p-text-color);
}

.files-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 0.75rem;
}

.file-item {
    border: 1px solid var(--p-surface-border);
    border-radius: var(--p-border-radius);
    padding: 1rem;
    background: var(--p-surface-50);
}

.file-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
}

.file-name {
    font-weight: 500;
    color: var(--p-text-color);
    flex: 1;
}

.file-actions {
    display: flex;
    gap: 0.5rem;
    flex-shrink: 0;
}

.file-preview {
    margin-top: 1rem;
    border: 1px solid var(--p-surface-border);
    border-radius: var(--p-border-radius);
    background: var(--p-surface-0);
    overflow: hidden;
}

.preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--p-surface-100);
    border-bottom: 1px solid var(--p-surface-border);
}

.preview-header h5 {
    margin: 0;
    color: var(--p-text-color);
    font-size: 0.9rem;
}

.preview-content {
    max-height: 300px;
    overflow-y: auto;
}

.loading-preview {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
    gap: 0.5rem;
}

.loading-preview p {
    margin: 0;
    color: var(--p-text-muted-color);
    font-size: 0.9rem;
}

.preview-error {
    padding: 1rem;
}

.code-preview {
    background: var(--p-surface-0);
    padding: 1rem;
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-size: 0.85rem;
    line-height: 1.4;
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
    color: var(--p-text-color);
}

.no-files {
    margin-top: 0.5rem;
}

.feedback-section {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 2px solid var(--p-surface-300);
}

.feedback-container {
    background: var(--p-surface-0);
    border: 1px solid var(--p-surface-200);
    border-radius: var(--p-border-radius);
    padding: 1rem;
    margin-top: 0.75rem;
}

.feedback-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0.75rem;
}

.feedback-item {
    border-radius: var(--p-border-radius);
    border: 1px solid var(--p-surface-200);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.feedback-accept {
    border-left: 4px solid var(--p-green-500);
    background: var(--p-green-50);
}

.feedback-reject {
    border-left: 4px solid var(--p-red-500);
    background: var(--p-red-50);
}

.feedback-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.feedback-message {
    font-size: 0.9rem;
    line-height: 1.4;
    color: var(--p-text-color);
}

.no-feedback {
    margin-top: 0.75rem;
}

.submission-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
}

.problem-dialog-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
}

.problem-description-section h3 {
    color: var(--p-text-color);
    margin-bottom: 1rem;
    font-size: 1.2rem;
}

.problem-description-text {
    background: var(--p-surface-100);
    padding: 1.5rem;
    border-radius: var(--p-border-radius);
    border-left: 4px solid var(--p-primary-color);
    color: var(--p-text-color);
    line-height: 1.8;
    font-size: 1rem;
    white-space: pre-wrap;
}

.submission-form-section {
    border-top: 2px solid var(--p-surface-border);
    padding-top: 1.5rem;
}

.submission-form-section h3 {
    color: var(--p-text-color);
    margin-bottom: 1.5rem;
    font-size: 1.2rem;
}

.submission-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.form-label {
    font-weight: 600;
    color: var(--p-text-color);
    font-size: 0.9rem;
}

.form-control {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #ddd;
    border-radius: 6px;
    font-size: 0.9rem;
    background: white;
    color: #333;
    resize: vertical;
    font-family: inherit;
    transition: border-color 0.2s;
}

.form-control:focus {
    outline: none;
    border-color: #007bff;
}

.file-upload {
    position: relative;
}

.file-input {
    display: none;
}

.file-upload-label {
    display: block;
    padding: 1rem;
    border: 2px dashed var(--p-surface-border);
    border-radius: var(--p-border-radius);
    text-align: center;
    cursor: pointer;
    transition: border-color 0.2s;
    color: var(--p-text-muted-color);
    background: var(--p-surface-50);
}

.file-upload-label:hover {
    border-color: var(--p-primary-color);
    color: var(--p-primary-color);
    background: var(--p-surface-100);
}

.help-text {
    font-size: 0.8rem;
    color: var(--p-text-muted-color);
    font-style: italic;
}

.selected-files {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 1rem;
}

.file-item-upload {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: var(--p-surface-100);
    border: 1px solid var(--p-surface-border);
    border-radius: var(--p-border-radius);
}

.file-item-upload .file-name {
    font-size: 0.9rem;
    color: var(--p-text-color);
    flex: 1;
}

.file-remove {
    background: none;
    border: none;
    color: var(--p-red-500);
    font-size: 1.2rem;
    cursor: pointer;
    padding: 0;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s;
}

.file-remove:hover {
    background: var(--p-red-500);
    color: white;
}

.form-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: 1rem;
    border-top: 1px solid var(--p-surface-border);
}

@media (max-width: 768px) {
    .header {
        flex-direction: column;
        gap: 1rem;
        align-items: stretch;
    }

    .stats {
        justify-content: center;
        flex-wrap: wrap;
    }

    .problems-grid {
        grid-template-columns: 1fr;
    }

    .problem-header {
        flex-direction: column;
        gap: 0.5rem;
        align-items: stretch;
    }

    .problem-badges {
        align-items: flex-start;
    }

    .submission-header {
        flex-direction: column;
        gap: 0.5rem;
        align-items: stretch;
    }

    .problem-actions,
    .submission-actions {
        flex-direction: column;
    }

    .file-info {
        flex-direction: column;
        gap: 0.75rem;
        align-items: stretch;
    }

    .file-actions {
        justify-content: flex-start;
    }

    .problem-dialog-content {
        gap: 1.5rem;
    }

    .problem-description-text {
        padding: 1rem;
    }

    .file-item-upload {
        flex-direction: column;
        gap: 0.5rem;
        align-items: stretch;
    }

    .form-actions {
        justify-content: stretch;
    }
}
</style>
