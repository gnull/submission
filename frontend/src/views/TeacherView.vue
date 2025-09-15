<template>
  <div class="teacher-view">
    <div class="header">
      <h1>Панель учителя</h1>
      <button @click="showCreateProblem = true" class="btn btn-success">
        ✨ Создать новую задачу
      </button>
    </div>

    <!-- Success/Error Messages -->
    <div v-if="message" :class="messageClass" class="alert">
      {{ message }}
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="loading">
      Загрузка...
    </div>

    <!-- Create Problem Modal -->
    <div v-if="showCreateProblem" class="modal-overlay" @click="closeCreateProblem">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2>Создать новую задачу</h2>
          <button @click="closeCreateProblem" class="btn-close">&times;</button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="createProblem">
            <div class="form-group">
              <label class="form-label">Название задачи</label>
              <input
                v-model="newProblem.name"
                type="text"
                class="form-control"
                required
                placeholder="Введите название задачи..."
              />
            </div>
            <div class="form-group">
              <label class="form-label">Описание задачи</label>
              <textarea
                v-model="newProblem.desc"
                class="form-control"
                rows="8"
                required
                placeholder="Напишите подробное описание задачи..."
              ></textarea>
            </div>
            <div class="modal-actions">
              <button type="button" @click="closeCreateProblem" class="btn btn-secondary">
                Отмена
              </button>
              <button type="submit" class="btn btn-success" :disabled="creating">
                {{ creating ? 'Создаём...' : 'Создать задачу' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Problems List -->
    <div v-if="!loading" class="problems-section">
      <h2>Ваши задачи</h2>
      <div v-if="problems.length === 0" class="empty-state">
        <p>Пока не создано ни одной задачи. Создайте первую задачу для начала!</p>
      </div>
      <div v-else class="problems-grid">
        <div v-for="problem in problems" :key="problem.id" class="problem-card">
          <div class="problem-header">
            <h3>{{ problem.name }}</h3>
            <div class="problem-stats">
              <span class="submissions-count">
                {{ getSubmissionCount(problem.id) }} работ
              </span>
            </div>
          </div>
          <p class="problem-description">{{ truncateText(problem.desc, 150) }}</p>
          <div class="problem-actions">
            <button @click="viewProblemSubmissions(problem.id)" class="btn btn-primary">
              Просмотреть работы
            </button>
            <router-link :to="`/problem/${problem.id}`" class="btn btn-secondary">
              Открыть задачу
            </router-link>
          </div>
        </div>
      </div>
    </div>

    <!-- Submissions Section -->
    <div v-if="selectedProblemSubmissions" class="submissions-section">
      <div class="submissions-header">
        <h2>Работы по задаче "{{ getSelectedProblemName() }}"</h2>
        <button @click="selectedProblemSubmissions = null" class="btn btn-secondary">
          Назад к задачам
        </button>
      </div>

      <div v-if="selectedProblemSubmissions.length === 0" class="empty-state">
        <p>Пока нет работ по этой задаче.</p>
      </div>

      <div v-else class="submissions-list">
        <div v-for="submission in selectedProblemSubmissions" :key="submission.id" class="submission-card">
          <div class="submission-header">
            <h4>Работа #{{ submission.id }}</h4>
            <div class="submission-status">
              <span :class="submission.status.accepted ? 'status-accepted' : 'status-pending'">
                {{ submission.status.accepted ? '✅ Принято' : '⏳ На проверке' }}
              </span>
            </div>
          </div>

          <p class="submission-comment">{{ submission.comment || 'Комментарий не добавлен' }}</p>

          <div class="submission-files">
            <strong>Файлы:</strong>
            <div class="file-list">
              <span v-for="file in submission.files" :key="file.id" class="file-tag">
                {{ file.name }}
              </span>
            </div>
          </div>

          <div class="submission-feedback">
            <div class="feedback-header">
              <strong>Обратная связь:</strong>
              <div class="feedback-actions">
                <button @click="openFeedbackModal(submission.id, 1)" class="btn btn-success btn-sm">
                  ✅ Принять
                </button>
                <button @click="openFeedbackModal(submission.id, 0)" class="btn btn-danger btn-sm">
                  ❌ Отклонить
                </button>
              </div>
            </div>
            <div v-if="submission.status.feedbacks.length > 0" class="feedback-list">
              <div
                v-for="feedback in submission.status.feedbacks"
                :key="feedback.id"
                :class="feedback.grade === 1 ? 'feedback-accept' : 'feedback-reject'"
                class="feedback-item"
              >
                <div class="feedback-grade">
                  {{ feedback.grade === 1 ? 'Принять' : 'Отклонить' }}
                </div>
                <div v-if="feedback.message" class="feedback-message">
                  {{ feedback.message }}
                </div>
              </div>
            </div>
            <div v-else class="no-feedback">
              Обратной связи пока нет
            </div>
          </div>

          <div class="submission-actions">
            <router-link :to="`/submission/${submission.id}`" class="btn btn-secondary">
              Подробности
            </router-link>
          </div>
        </div>
      </div>
    </div>

    <!-- Feedback Modal -->
    <div v-if="showFeedbackModal" class="modal-overlay" @click="closeFeedbackModal">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2>{{ feedbackForm.grade === 1 ? '✅ Принять' : '❌ Отклонить' }} работу</h2>
          <button @click="closeFeedbackModal" class="btn-close">&times;</button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="submitFeedback">
            <div class="form-group">
              <label class="form-label">
                Сообщение (необязательно)
              </label>
              <textarea
                v-model="feedbackForm.message"
                class="form-control"
                rows="4"
                placeholder="Добавьте сообщение, чтобы объяснить вашу оценку..."
              ></textarea>
            </div>
            <div class="modal-actions">
              <button type="button" @click="closeFeedbackModal" class="btn btn-secondary">
                Отмена
              </button>
              <button type="submit" class="btn" :class="feedbackForm.grade === 1 ? 'btn-success' : 'btn-danger'" :disabled="submittingFeedback">
                {{ submittingFeedback ? 'Отправляем...' : (feedbackForm.grade === 1 ? 'Принять' : 'Отклонить') }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { apiService } from '../api'
import type { Problem, Submission, CreateProblem } from '../types'

// Reactive state
const problems = ref<Problem[]>([])
const allSubmissions = ref<Submission[]>([])
const selectedProblemSubmissions = ref<Submission[] | null>(null)
const selectedProblemId = ref<number | null>(null)
const loading = ref(true)
const creating = ref(false)
const showCreateProblem = ref(false)
const showFeedbackModal = ref(false)
const submittingFeedback = ref(false)
const message = ref('')
const messageType = ref<'success' | 'error'>('success')

const newProblem = ref<CreateProblem>({
  name: '',
  desc: ''
})

const feedbackForm = ref({
  submissionId: 0,
  grade: 0,
  message: ''
})

// Computed
const messageClass = computed(() => messageType.value === 'success' ? 'alert-success' : 'alert-error')

// Methods
const loadData = async () => {
  try {
    loading.value = true

    // Load problems first
    console.log('Loading problems...')
    try {
      const problemsData = await apiService.getProblems()
      problems.value = problemsData
      console.log('Problems loaded successfully:', problemsData)
    } catch (error) {
      console.error('Error loading problems:', error)
      showMessage('Не удалось загрузить задачи', 'error')
      return
    }

    // Load submissions second
    console.log('Loading submissions...')
    try {
      const submissionsData = await apiService.getSubmissions()
      allSubmissions.value = submissionsData
      console.log('Submissions loaded successfully:', submissionsData)
    } catch (error) {
      console.error('Error loading submissions:', error)
      showMessage('Не удалось загрузить работы', 'error')
      return
    }

  } catch (error) {
    showMessage('Не удалось загрузить данные', 'error')
    console.error('Error loading data:', error)
  } finally {
    loading.value = false
  }
}

const createProblem = async () => {
  if (!newProblem.value.name.trim() || !newProblem.value.desc.trim()) {
    showMessage('Пожалуйста, заполните все поля', 'error')
    return
  }

  try {
    creating.value = true
    await apiService.createProblem(newProblem.value)
    showMessage('Задача успешно создана!', 'success')
    closeCreateProblem()
    await loadData() // Reload data
  } catch (error) {
    showMessage('Не удалось создать задачу', 'error')
    console.error('Error creating problem:', error)
  } finally {
    creating.value = false
  }
}

const viewProblemSubmissions = (problemId: number) => {
  selectedProblemId.value = problemId
  selectedProblemSubmissions.value = allSubmissions.value.filter(s => s.problem === problemId)
}

const openFeedbackModal = (submissionId: number, grade: number) => {
  feedbackForm.value = {
    submissionId,
    grade,
    message: ''
  }
  showFeedbackModal.value = true
}

const closeFeedbackModal = () => {
  showFeedbackModal.value = false
  feedbackForm.value = {
    submissionId: 0,
    grade: 0,
    message: ''
  }
}

const submitFeedback = async () => {
  try {
    submittingFeedback.value = true
    const feedbackData = {
      grade: feedbackForm.value.grade,
      message: feedbackForm.value.message.trim() || undefined
    }
    await apiService.createFeedback(feedbackForm.value.submissionId, feedbackData)
    showMessage(`Оценка "${feedbackForm.value.grade === 1 ? 'принято' : 'отклонено'}" успешно добавлена!`, 'success')
    closeFeedbackModal()
    await loadData() // Reload to get updated feedback
    // Update the selected submissions if viewing them
    if (selectedProblemId.value) {
      viewProblemSubmissions(selectedProblemId.value)
    }
  } catch (error) {
    showMessage('Не удалось добавить оценку', 'error')
    console.error('Error giving feedback:', error)
  } finally {
    submittingFeedback.value = false
  }
}

const getSubmissionCount = (problemId: number): number => {
  return allSubmissions.value.filter(s => s.problem === problemId).length
}

const getSelectedProblemName = (): string => {
  if (!selectedProblemId.value) return ''
  const problem = problems.value.find(p => p.id === selectedProblemId.value)
  return problem?.name || ''
}

const truncateText = (text: string, length: number): string => {
  if (text.length <= length) return text
  return text.substring(0, length) + '...'
}

const closeCreateProblem = () => {
  showCreateProblem.value = false
  newProblem.value = { name: '', desc: '' }
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
.teacher-view {
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

.problems-section {
  margin-bottom: 3rem;
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
}

.problem-stats {
  display: flex;
  gap: 0.5rem;
}

.submissions-count {
  background-color: #e9ecef;
  color: #495057;
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

.submissions-section {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.submissions-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid #e9ecef;
}

.submissions-header h2 {
  margin: 0;
  color: #2c3e50;
}

.submissions-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
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
  background-color: #fff3cd;
  color: #856404;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
}

.submission-comment {
  color: #666;
  font-style: italic;
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
  padding: 1rem;
  background: white;
  border-radius: 6px;
}

.feedback-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.feedback-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-sm {
  padding: 0.25rem 0.5rem;
  font-size: 0.8rem;
}

.feedback-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
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
}

.submission-actions {
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
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e9ecef;
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

.modal-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
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

  .submissions-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }

  .submission-header {
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }

  .feedback-header {
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }

  .modal {
    width: 95%;
    margin: 1rem;
  }
}
</style>
