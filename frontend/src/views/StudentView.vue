<template>
    <div class="student-view">
        <!-- Header Section -->
        <div class="header">
            <h1>Панель ученика</h1>
            <div class="stats">
                <Card class="stat-card">
                    <template #content>
                        <div class="stat-item">
                            <span class="stat-number">{{
                                acceptedProblems
                            }}</span>
                            <span class="stat-label">Принято</span>
                        </div>
                    </template>
                </Card>
                <Card class="stat-card">
                    <template #content>
                        <div class="stat-item">
                            <span class="stat-number">{{ totalAttempts }}</span>
                            <span class="stat-label">Всего попыток</span>
                        </div>
                    </template>
                </Card>
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
                                @click="$router.push(`/problem/${problem.id}`)"
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
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { apiService } from "../api";
import type { ProblemWithStats, Submission, Problem } from "../types";

// Reactive state
const problemsWithStats = ref<ProblemWithStats[]>([]);
const allSubmissions = ref<Submission[]>([]);
const selectedSubmissions = ref<Submission[]>([]);
const selectedProblemName = ref("");
const loading = ref(true);
const showSubmissions = ref(false);
const message = ref("");
const messageType = ref<"success" | "error" | "info" | "warn">("success");

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

.stat-card {
    min-width: 120px;
}

.stat-item {
    text-align: center;
    padding: 0.5rem;
}

.stat-number {
    display: block;
    font-size: 2rem;
    font-weight: bold;
    color: var(--p-green-500);
    margin-bottom: 0.25rem;
}

.stat-label {
    display: block;
    font-size: 0.9rem;
    color: var(--p-text-muted-color);
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
}

.problem-card:not(.problem-accepted) {
    border-left: 4px solid var(--p-primary-color);
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
}
</style>
