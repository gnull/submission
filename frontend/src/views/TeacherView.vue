<template>
    <div class="teacher-view">
        <!-- Toast for notifications -->
        <Toast />

        <!-- Header -->
        <Panel header="Панель учителя" class="mb-4">
            <template #headerActions>
                <Button
                    label="✨ Создать новую задачу"
                    icon="pi pi-plus"
                    @click="showCreateProblem = true"
                    severity="success"
                />
            </template>
        </Panel>

        <!-- Loading State -->
        <div
            v-if="loading"
            class="flex justify-content-center align-items-center p-4"
        >
            <ProgressSpinner />
            <span class="ml-2">Загрузка...</span>
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
        <Panel v-if="!loading" header="Ваши задачи" class="mb-4">
            <div v-if="problems.length === 0" class="text-center p-4">
                <p class="text-muted-color">
                    Пока не создано ни одной задачи. Создайте первую задачу для
                    начала!
                </p>
            </div>

            <div v-else class="problems-grid">
                <Card
                    v-for="problem in problems"
                    :key="problem.id"
                    class="problem-card"
                >
                    <template #content>
                        <div class="problem-content">
                            <div class="problem-header">
                                <h3 class="m-0 text-color">
                                    {{ problem.name }}
                                </h3>
                                <Badge
                                    :value="
                                        getSubmissionCount(problem.id) +
                                        ' работ'
                                    "
                                    severity="secondary"
                                />
                            </div>

                            <p class="problem-description">
                                {{ truncateText(problem.desc, 150) }}
                            </p>

                            <div class="problem-actions">
                                <Button
                                    label="Просмотреть работы"
                                    icon="pi pi-list"
                                    @click="viewProblemSubmissions(problem.id)"
                                    class="flex-1"
                                />
                                <Button
                                    label="Открыть задачу"
                                    icon="pi pi-external-link"
                                    severity="secondary"
                                    @click="
                                        $router.push(`/problem/${problem.id}`)
                                    "
                                    class="flex-1"
                                />
                            </div>
                        </div>
                    </template>
                </Card>
            </div>
        </Panel>

        <!-- Submissions Section -->
        <Panel
            v-if="selectedProblemSubmissions"
            :header="submissionsPanelHeader"
        >
            <template #headerActions>
                <Button
                    label="Назад к задачам"
                    icon="pi pi-arrow-left"
                    severity="secondary"
                    @click="selectedProblemSubmissions = null"
                />
            </template>

            <div
                v-if="selectedProblemSubmissions.length === 0"
                class="text-center p-4"
            >
                <p class="text-muted-color">Пока нет работ по этой задаче.</p>
            </div>

            <div v-else class="submissions-list">
                <Card
                    v-for="submission in selectedProblemSubmissions"
                    :key="submission.id"
                    class="submission-card"
                >
                    <template #content>
                        <div class="submission-content">
                            <div class="submission-header">
                                <h4 class="m-0">Работа #{{ submission.id }}</h4>
                                <Tag
                                    :value="
                                        submission.status.accepted
                                            ? '✅ Принято'
                                            : '⏳ На проверке'
                                    "
                                    :severity="
                                        submission.status.accepted
                                            ? 'success'
                                            : 'warning'
                                    "
                                />
                            </div>

                            <p class="submission-comment">
                                {{
                                    submission.comment ||
                                    "Комментарий не добавлен"
                                }}
                            </p>

                            <div class="submission-files">
                                <strong>Файлы:</strong>
                                <div class="flex flex-wrap gap-1 mt-2">
                                    <Chip
                                        v-for="file in submission.files"
                                        :key="file.id"
                                        :label="file.name"
                                    />
                                </div>
                            </div>

                            <Divider />

                            <div class="submission-feedback">
                                <div class="feedback-header">
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
                                    v-if="
                                        submission.status.feedbacks.length > 0
                                    "
                                    class="feedback-list"
                                >
                                    <Card
                                        v-for="feedback in submission.status
                                            .feedbacks"
                                        :key="feedback.id"
                                        class="feedback-item"
                                    >
                                        <template #content>
                                            <div class="feedback-item-content">
                                                <div
                                                    class="flex align-items-center gap-2"
                                                >
                                                    <Tag
                                                        :value="
                                                            feedback.grade === 1
                                                                ? 'Принять'
                                                                : 'Отклонить'
                                                        "
                                                        :severity="
                                                            feedback.grade === 1
                                                                ? 'success'
                                                                : 'danger'
                                                        "
                                                    />
                                                </div>
                                                <div
                                                    v-if="feedback.message"
                                                    class="text-sm"
                                                >
                                                    {{ feedback.message }}
                                                </div>
                                            </div>
                                        </template>
                                    </Card>
                                </div>
                                <div
                                    v-else
                                    class="text-muted-color font-italic"
                                >
                                    Обратной связи пока нет
                                </div>
                            </div>

                            <div class="submission-actions">
                                <Button
                                    label="Подробности"
                                    icon="pi pi-info-circle"
                                    severity="secondary"
                                    @click="
                                        $router.push(
                                            `/submission/${submission.id}`,
                                        )
                                    "
                                />
                            </div>
                        </div>
                    </template>
                </Card>
            </div>
        </Panel>

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

.problems-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1.5rem;
}

.problem-card {
    height: 100%;
}

.problem-content {
    padding: 0.75rem;
}

.problem-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.5rem;
}

.problem-description {
    color: #666;
    line-height: 1.5;
    margin-bottom: 0.75rem;
}

.problem-actions {
    display: flex;
    gap: 0.5rem;
}

.submissions-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.submission-card {
    background: #f8f9fa;
}

.submission-content {
    padding: 0.75rem;
}

.submission-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
}

.submission-comment {
    color: #666;
    font-style: italic;
    margin-bottom: 0.5rem;
}

.submission-files {
    margin-bottom: 0.5rem;
}

.submission-feedback {
    margin-bottom: 0.75rem;
}

.feedback-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
}

.feedback-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.feedback-item {
    padding: 0.25rem;
}

.feedback-item-content {
    padding: 0.5rem;
}

.submission-actions {
    display: flex;
    gap: 0.5rem;
}

@media (max-width: 768px) {
    .problems-grid {
        grid-template-columns: 1fr;
    }

    .teacher-view {
        padding: 0.5rem;
    }
}
</style>
