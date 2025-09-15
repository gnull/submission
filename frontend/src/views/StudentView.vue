<template>
  <div class="student-view">
    <div class="header">
      <h1>Панель ученика</h1>
      <div class="stats">
        <div class="stat-item">
          <span class="stat-number">{{ acceptedProblems }}</span>
          <span class="stat-label">Принято</span>
        </div>
        <div class="stat-item">
          <span class="stat-number">{{ totalAttempts }}</span>
          <span class="stat-label">Всего попыток</span>
        </div>
      </div>
    </div>

    <!-- Success/Error Messages -->
    <div v-if="message" :class="messageClass" class="alert">
      {{ message }}
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="loading">
      Загружаем задачи...
    </div>

    <!-- Problems List -->
    <div v-if="!loading" class="problems-section">
      <h2>Доступные задачи</h2>
      <div v-if="problemsWithStats.length === 0" class="empty-state">
        <p>Пока нет доступных задач. Заходите позже!</p>
      </div>
      <div v-else class="problems-grid">
        <div
          v-for="problem in problemsWithStats"
          :key="problem.id"
          :class="['problem-card', { 'problem-accepted': problem.accepted }]"
        >
          <div class="problem-header">
            <h3>{{ problem.name }}</h3>
            <div class="problem-badges">
              <span v-if="problem.accepted" class="badge-accepted">✅ Принято</span>
              <span class="problem-attempts">{{ problem.attempts }} попыток</span>
            </div>
          </div>
          <p class="problem-description">{{ truncateText(problem.desc, 150) }}</p>
          <div class="problem-actions">
            <router-link :to="`/problem/${problem.id}`" class="btn btn-primary">
              {{ problem.attempts > 0 ? 'Открыть и переотправить' : 'Открыть задачу' }}
            </router-link>
            <button
              v-if="problem.attempts > 0"
              @click="viewSubmissions(problem.id)"
              class="btn btn-secondary"
            >
              Мои работы
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- My Submissions Modal -->
    <div v-if="showSubmissions" class="modal-overlay" @click="closeSubmissions">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2>Мои работы по задаче "{{ selectedProblemName }}"</h2>
          <button @click="closeSubmissions" class="btn-close">&times;</button>
        </div>
        <div class="modal-body">
          <div v-if="selectedSubmissions.length === 0" class="empty-state">
            <p>Работы не найдены.</p>
          </div>
          <div v-else class="submissions-list">
            <div
              v-for="submission in selectedSubmissions"
              :key="submission.id"
              class="submission-card"
            >
              <div class="submission-header">
                <h4>Работа #{{ submission.id }}</h4>
                <div class="submission-status">
                  <span :class="submission.status.accepted ? 'status-accepted' : 'status-pending'">
                    {{ submission.status.accepted ? '✅ Принято' : '❌ Не принято' }}
                  </span>
                </div>
              </div>

              <p class="submission-comment">
                <strong>Комментарий:</strong> {{ submission.comment || 'Комментарий не добавлен' }}
              </p>

              <div class="submission-files">
                <strong>Отправленные файлы:</strong>
                <div class="file-list">
                  <span v-for="file in submission.files" :key="file.id" class="file-tag">
                    {{ file.name }}
                  </span>
                </div>
              </div>

              <div class="submission-feedback" v-if="submission.status.feedbacks.length > 0">
                <strong>Обратная связь:</strong>
                <div class="feedback-list">
                  <div
                    v-for="feedback in submission.status.feedbacks"
                    :key="feedback.id"
                    :class="feedback.grade === 1 ? 'feedback-accept' : 'feedback-reject'"
                    class="feedback-item"
                  >
                    <div class="feedback-grade">
                      {{ feedback.grade === 1 ? 'Принято' : 'Отклонено' }}
                    </div>
                    <div v-if="feedback.message" class="feedback-message">
                      {{ feedback.message }}
                    </div>
                  </div>
                </div>
              </div>
              <div v-else class="no-feedback">
                <em>Обратной связи пока нет - ждём проверки</em>
              </div>

              <div class="submission-actions">
                <router-link :to="`/submission/${submission.id}`" class="btn btn-sm btn-secondary">
                  Подробности
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { apiService } from '../api'
import type { ProblemWithStats, Submission, Problem } from '../types'

// Reactive state
const problemsWithStats = ref<ProblemWithStats[]>([])
const allSubmissions = ref<Submission[]>([])
const selectedSubmissions = ref<Submission[]>([])
const selectedProblemName = ref('')
const loading = ref(true)
const showSubmissions = ref(false)
const message = ref('')
const messageType = ref<'success' | 'error'>('success')

// Computed properties
const acceptedProblems = computed(() =>
  problemsWithStats.value.filter(p => p.accepted).length
)

const totalAttempts = computed(() =>
  problemsWithStats.value.reduce((sum, p) => sum + p.attempts, 0)
)

const messageClass = computed(() =>
  messageType.value === 'success' ? 'alert-success' : 'alert-error'
)

// Methods
const loadData = async () => {
  try {
    loading.value = true
    const [problems, submissions] = await Promise.all([
      apiService.getProblems(),
      apiService.getSubmissions()
    ])

    allSubmissions.value = submissions

    // Calculate stats for each problem
    problemsWithStats.value = problems.map(problem => {
      const problemSubmissions = submissions.filter(s => s.problem === problem.id)
      const attempts = problemSubmissions.length
      const accepted = problemSubmissions.some(s => s.status.accepted)

      return {
        ...problem,
        attempts,
        accepted,
      }
    })

  } catch (error) {
    showMessage('Не удалось загрузить задачи', 'error')
    console.error('Error loading data:', error)
  } finally {
    loading.value = false
  }
}

const viewSubmissions = (problemId: number) => {
  const problem = problemsWithStats.value.find(p => p.id === problemId)
  if (!problem) return

  selectedProblemName.value = problem.name
  selectedSubmissions.value = allSubmissions.value.filter(s => s.problem === problemId)
  showSubmissions.value = true
}

const closeSubmissions = () => {
  showSubmissions.value = false
  selectedSubmissions.value = []
  selectedProblemName.value = ''
}

const truncateText = (text: string, length: number): string => {
  if (text.length <= length) return text
  return text.substring(0, length) + '...'
}

const showMessage = (msg: string, type: 'success' | 'error') => {
  message.value = msg
  messageType.value = type
  setTimeout(() => {
    message.value = ''
  }, 5000)
}

// Lifecycle
onMounted(() => {
  loadData()
})
</script>

<style scoped>
.student-view {
  max-width: 1200px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 2px solid #e9ecef;
}

.header h1 {
  color: #2c3e50;
  margin: 0;
}

.stats {
  display: flex;
  gap: 2rem;
}

.stat-item {
  text-align: center;
  padding: 1rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  min-width: 100px;
}

.stat-number {
  display: block;
  font-size: 2rem;
  font-weight: bold;
  color: #28a745;
}

.stat-label {
  display: block;
  font-size: 0.9rem;
  color: #6c757d;
  margin-top: 0.25rem;
}

.problems-section h2 {
  margin-bottom: 1.5rem;
  color: #2c3e50;
}

.problems-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1.5rem;
}

.problem-card {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  border-left: 4px solid #007bff;
  transition: transform 0.2s, box-shadow 0.2s;
}

.problem-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.problem-accepted {
  border-left-color: #28a745 !important;
  background: linear-gradient(135deg, #f8fff9 0%, #ffffff 100%);
}

.problem-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.problem-header h3 {
  margin: 0;
  color: #2c3e50;
  flex: 1;
}

.problem-badges {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  align-items: flex-end;
}

.badge-accepted {
  background-color: #d4edda;
  color: #155724;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 500;
}

.problem-description {
  color: #666;
  line-height: 1.5;
  margin-bottom: 1.5rem;
}

.problem-actions {
  display: flex;
  gap: 0.5rem;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: #6c757d;
}

.empty-state p {
  font-size: 1.1rem;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 800px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e9ecef;
  position: sticky;
  top: 0;
  background: white;
}

.modal-header h2 {
  margin: 0;
  color: #2c3e50;
}

.btn-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #6c757d;
}

.modal-body {
  padding: 1.5rem;
}

.submissions-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.submission-card {
  background: #f8f9fa;
  padding: 1.5rem;
  border-radius: 8px;
  border-left: 4px solid #6c757d;
}

.submission-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.submission-header h4 {
  margin: 0;
  color: #2c3e50;
}

.status-accepted {
  background-color: #d4edda;
  color: #155724;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
}

.status-pending {
  background-color: #f8d7da;
  color: #721c24;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
}

.submission-comment {
  color: #666;
  margin-bottom: 1rem;
}

.submission-files {
  margin-bottom: 1rem;
}

.file-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

.file-tag {
  background-color: #e9ecef;
  color: #495057;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
}

.submission-feedback {
  margin-bottom: 1rem;
}

.feedback-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-top: 0.5rem;
}

.feedback-item {
  padding: 0.75rem;
  border-radius: 4px;
  border-left: 3px solid;
}

.feedback-accept {
  background-color: #d4edda;
  border-left-color: #28a745;
}

.feedback-reject {
  background-color: #f8d7da;
  border-left-color: #dc3545;
}

.feedback-grade {
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.feedback-accept .feedback-grade {
  color: #155724;
}

.feedback-reject .feedback-grade {
  color: #721c24;
}

.feedback-message {
  font-size: 0.9rem;
  line-height: 1.4;
  margin-top: 0.5rem;
}

.feedback-accept .feedback-message {
  color: #155724;
}

.feedback-reject .feedback-message {
  color: #721c24;
}

.no-feedback {
  color: #6c757d;
  font-style: italic;
  margin-top: 0.5rem;
}

.submission-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
}

.btn-sm {
  padding: 0.25rem 0.5rem;
  font-size: 0.8rem;
}

@media (max-width: 768px) {
  .header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }

  .stats {
    justify-content: center;
  }

  .problems-grid {
    grid-template-columns: 1fr;
  }

  .problem-header {
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }

  .problem-badges {
    align-items: flex-start;
  }

  .submission-header {
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }

  .modal {
    width: 95%;
    margin: 1rem;
  }

  .modal-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }

  .modal-header h2 {
    text-align: center;
  }
}
</style>
