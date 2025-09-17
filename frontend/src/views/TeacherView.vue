<template>
    <div class="teacher-view">
        <!-- Toast for notifications -->
        <Toast />

        <!-- Header Section -->
        <div class="header">
            <h1>Панель учителя</h1>
            <Button
                label="✨ Создать новую задачу"
                icon="pi pi-plus"
                @click="showCreateProblem = true"
                severity="success"
            />
        </div>

        <!-- Loading State -->
        <div v-if="loading" class="loading-container">
            <ProgressSpinner />
            <p>Загружаем задачи...</p>
        </div>

        <!-- Create Problem Modal -->
        <Dialog
            v-model:visible="showCreateProblem"
            modal
            header="Создать новую задачу"
            :style="{ width: '600px' }"
            @hide="closeCreateProblem"
        >
            <form @submit.prevent="createProblem" class="p-fluid">
                <div class="field mb-4">
                    <FloatLabel>
                        <InputText
                            id="problem-name"
                            v-model="newProblem.name"
                            required
                        />
                        <label for="problem-name">Название задачи</label>
                    </FloatLabel>
                </div>

                <div class="field mb-4">
                    <FloatLabel>
                        <Textarea
                            id="problem-desc"
                            v-model="newProblem.desc"
                            rows="8"
                            required
                        />
                        <label for="problem-desc">Описание задачи</label>
                    </FloatLabel>
                </div>

                <div class="flex justify-content-end gap-2">
                    <Button
                        label="Отмена"
                        severity="secondary"
                        @click="closeCreateProblem"
                        type="button"
                    />
                    <Button
                        :label="creating ? 'Создаём...' : 'Создать задачу'"
                        severity="success"
                        type="submit"
                        :loading="creating"
                        :disabled="creating"
                    />
                </div>
            </form>
        </Dialog>

        <!-- Problems List -->
        <div v-if="!loading" class="problems-section">
            <h2>Ваши задачи</h2>
            <div v-if="problems.length === 0" class="empty-state">
                <Message severity="info" :closable="false">
                    Пока не создано ни одной задачи. Создайте первую задачу для
                    начала!
                </Message>
            </div>

            <div v-else class="problems-grid">
                <Card
                    v-for="problem in problems"
                    :key="problem.id"
                    class="problem-card"
                >
                    <template #header>
                        <div class="problem-header">
                            <h3>{{ problem.name }}</h3>
                            <div class="problem-badges">
                                <Tag
                                    severity="secondary"
                                    :value="
                                        getSubmissionCount(problem.id) +
                                        ' работ'
                                    "
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
                                label="Просмотреть работы"
                                icon="pi pi-list"
                                @click="viewProblemSubmissions(problem.id)"
                                class="p-button-primary"
                            />
                            <Button
                                label="Открыть задачу"
                                icon="pi pi-external-link"
                                severity="secondary"
                                @click="$router.push(`/problem/${problem.id}`)"
                            />
                        </div>
                    </template>
                </Card>
            </div>
        </div>

        <!-- My Submissions Dialog -->
        <Dialog
            v-model:visible="showSubmissionsDialog"
            :header="submissionsPanelHeader"
            modal
            :style="{ width: '90vw', maxWidth: '1000px' }"
            :maximizable="true"
        >
            <template #header>
                <div
                    class="flex justify-content-between align-items-center w-full"
                >
                    <span>{{ submissionsPanelHeader }}</span>
                    <Button
                        label="Назад к задачам"
                        icon="pi pi-arrow-left"
                        severity="secondary"
                        @click="
                            showSubmissionsDialog = false;
                            selectedProblemSubmissions = null;
                        "
                        size="small"
                    />
                </div>
            </template>

            <div
                v-if="
                    selectedProblemSubmissions &&
                    selectedProblemSubmissions.length === 0
                "
                class="empty-state"
            >
                <Message severity="info" :closable="false">
                    Пока нет работ по этой задаче.
                </Message>
            </div>

            <div
                v-else-if="selectedProblemSubmissions"
                class="submissions-list"
            >
                <Card
                    v-for="submission in selectedProblemSubmissions"
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
                                <div class="file-list">
                                    <Tag
                                        v-for="file in submission.files"
                                        :key="file.id"
                                        :value="file.name"
                                        severity="secondary"
                                    />
                                </div>
                            </div>

                            <div class="feedback-section">
                                <div class="feedback-actions">
                                    <strong>Обратная связь:</strong>
                                    <div class="flex gap-2">
                                        <Button
                                            label="✅ Принять"
                                            severity="success"
                                            size="small"
                                            @click="
                                                openFeedbackModal(
                                                    submission.id,
                                                    1,
                                                )
                                            "
                                        />
                                        <Button
                                            label="❌ Отклонить"
                                            severity="danger"
                                            size="small"
                                            @click="
                                                openFeedbackModal(
                                                    submission.id,
                                                    0,
                                                )
                                            "
                                        />
                                    </div>
                                </div>

                                <div
                                    class="feedback-container"
                                    v-if="
                                        submission.status.feedbacks.length > 0
                                    "
                                >
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
                    <template #footer>
                        <div class="submission-actions">
                            <Button
                                label="Подробности"
                                @click="
                                    $router.push(`/submission/${submission.id}`)
                                "
                                icon="pi pi-info-circle"
                                size="small"
                                severity="secondary"
                            />
                        </div>
                    </template>
                </Card>
            </div>
        </Dialog>

        <!-- Feedback Modal -->
        <Dialog
            v-model:visible="showFeedbackModal"
            modal
            :header="
                feedbackForm.grade === 1
                    ? '✅ Принять работу'
                    : '❌ Отклонить работу'
            "
            :style="{ width: '500px' }"
            @hide="closeFeedbackModal"
        >
            <form @submit.prevent="submitFeedback" class="p-fluid">
                <div class="field mb-4">
                    <FloatLabel>
                        <Textarea
                            id="feedback-message"
                            v-model="feedbackForm.message"
                            rows="4"
                        />
                        <label for="feedback-message"
                            >Сообщение (необязательно)</label
                        >
                    </FloatLabel>
                </div>

                <div class="flex justify-content-end gap-2">
                    <Button
                        label="Отмена"
                        severity="secondary"
                        @click="closeFeedbackModal"
                        type="button"
                    />
                    <Button
                        :label="
                            submittingFeedback
                                ? 'Отправляем...'
                                : feedbackForm.grade === 1
                                  ? 'Принять'
                                  : 'Отклонить'
                        "
                        :severity="
                            feedbackForm.grade === 1 ? 'success' : 'danger'
                        "
                        type="submit"
                        :loading="submittingFeedback"
                        :disabled="submittingFeedback"
                    />
                </div>
            </form>
        </Dialog>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useToast } from "primevue/usetoast";
import { useRouter } from "vue-router";
import { apiService } from "../api";
import type { Problem, Submission, CreateProblem } from "../types";

// Router and Toast
const router = useRouter();
const toast = useToast();

// Reactive state
const problems = ref<Problem[]>([]);
const allSubmissions = ref<Submission[]>([]);
const selectedProblemSubmissions = ref<Submission[] | null>(null);
const selectedProblemId = ref<number | null>(null);
const loading = ref(true);
const creating = ref(false);
const showCreateProblem = ref(false);
const showFeedbackModal = ref(false);
const submittingFeedback = ref(false);
const showSubmissionsDialog = ref(false);

const newProblem = ref<CreateProblem>({
    name: "",
    desc: "",
});

const feedbackForm = ref({
    submissionId: 0,
    grade: 0,
    message: "",
});

// Computed
const submissionsPanelHeader = computed(
    () => `Работы по задаче "${getSelectedProblemName()}"`,
);

// Methods
const loadData = async () => {
    try {
        loading.value = true;

        // Load problems first
        console.log("Loading problems...");
        try {
            const problemsData = await apiService.getProblems();
            problems.value = problemsData;
            console.log("Problems loaded successfully:", problemsData);
        } catch (error) {
            console.error("Error loading problems:", error);
            showMessage("Не удалось загрузить задачи", "error");
            return;
        }

        // Load submissions second
        console.log("Loading submissions...");
        try {
            const submissionsData = await apiService.getSubmissions();
            allSubmissions.value = submissionsData;
            console.log("Submissions loaded successfully:", submissionsData);
        } catch (error) {
            console.error("Error loading submissions:", error);
            showMessage("Не удалось загрузить работы", "error");
            return;
        }
    } catch (error) {
        showMessage("Не удалось загрузить данные", "error");
        console.error("Error loading data:", error);
    } finally {
        loading.value = false;
    }
};

const createProblem = async () => {
    if (!newProblem.value.name.trim() || !newProblem.value.desc.trim()) {
        showMessage("Пожалуйста, заполните все поля", "error");
        return;
    }

    try {
        creating.value = true;
        await apiService.createProblem(newProblem.value);
        showMessage("Задача успешно создана!", "success");
        closeCreateProblem();
        await loadData(); // Reload data
    } catch (error) {
        showMessage("Не удалось создать задачу", "error");
        console.error("Error creating problem:", error);
    } finally {
        creating.value = false;
    }
};

const viewProblemSubmissions = (problemId: number) => {
    selectedProblemId.value = problemId;
    selectedProblemSubmissions.value = allSubmissions.value.filter(
        (s) => s.problem === problemId,
    );
    showSubmissionsDialog.value = true;
};

const openFeedbackModal = (submissionId: number, grade: number) => {
    feedbackForm.value = {
        submissionId,
        grade,
        message: "",
    };
    showFeedbackModal.value = true;
};

const closeFeedbackModal = () => {
    showFeedbackModal.value = false;
    feedbackForm.value = {
        submissionId: 0,
        grade: 0,
        message: "",
    };
};

const submitFeedback = async () => {
    try {
        submittingFeedback.value = true;
        const feedbackData = {
            grade: feedbackForm.value.grade,
            message: feedbackForm.value.message.trim() || undefined,
        };
        await apiService.createFeedback(
            feedbackForm.value.submissionId,
            feedbackData,
        );
        showMessage(
            `Оценка "${feedbackForm.value.grade === 1 ? "принято" : "отклонено"}" успешно добавлена!`,
            "success",
        );
        closeFeedbackModal();
        await loadData(); // Reload to get updated feedback
        // Update the selected submissions if viewing them
        if (selectedProblemId.value) {
            viewProblemSubmissions(selectedProblemId.value);
        }
    } catch (error) {
        showMessage("Не удалось добавить оценку", "error");
        console.error("Error giving feedback:", error);
    } finally {
        submittingFeedback.value = false;
    }
};

const getSubmissionCount = (problemId: number): number => {
    return allSubmissions.value.filter((s) => s.problem === problemId).length;
};

const getSelectedProblemName = (): string => {
    if (!selectedProblemId.value) return "";
    const problem = problems.value.find(
        (p) => p.id === selectedProblemId.value,
    );
    return problem?.name || "";
};

const truncateText = (text: string, length: number): string => {
    if (text.length <= length) return text;
    return text.substring(0, length) + "...";
};

const closeCreateProblem = () => {
    showCreateProblem.value = false;
    newProblem.value = { name: "", desc: "" };
};

const showMessage = (msg: string, type: "success" | "error") => {
    toast.add({
        severity: type,
        summary: type === "success" ? "Успех" : "Ошибка",
        detail: msg,
        life: 5000,
    });
};

// Lifecycle
onMounted(() => {
    loadData();
});
</script>

<style scoped>
.teacher-view {
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

.empty-state {
    text-align: center;
    padding: 2rem;
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
    border-left: 4px solid var(--p-primary-color);
}

.problem-card:hover {
    transform: translateY(-2px);
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

.problem-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
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

.file-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.5rem;
}

.feedback-section {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 2px solid var(--p-surface-300);
}

.feedback-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
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

@media (max-width: 768px) {
    .header {
        flex-direction: column;
        gap: 1rem;
        align-items: stretch;
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

    .teacher-view {
        padding: 0.5rem;
    }
}
</style>
