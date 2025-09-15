<template>
  <div class="problem-detail">
    <!-- Loading State -->
    <div v-if="loading" class="loading">
      Загружаем задачу...
    </div>

    <!-- Error State -->
    <div v-if="error" class="alert alert-error">
      {{ error }}
    </div>

    <!-- Validation Error -->
    <div v-if="validationError" class="alert alert-error">
      {{ validationError }}
    </div>

    <!-- Problem Content -->
    <div v-if="problem && !loading" class="problem-content">
      <div class="problem-header">
        <div class="breadcrumb">
          <router-link to="/student" class="breadcrumb-link">← Назад к задачам</router-link>
        </div>
        <h1>{{ problem.name }}</h1>
        <div class="problem-meta">
          <span v-if="isAccepted" class="badge-accepted">✅ Принято</span>
          <span class="attempts-badge">{{ attemptCount }} попыток</span>
        </div>
      </div>

      <div class="problem-description">
        <h2>Описание задачи</h2>
        <div class="description-content">
          {{ problem.desc }}
        </div>
      </div>

      <!-- Submission Form -->
      <div class="submission-section">
        <h2>Отправить решение</h2>
        <form @submit.prevent="submitSolution" class="submission-form">
          <div class="form-group">
            <label class="form-label">Комментарий (необязательно)</label>
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
              <div class="file-upload-label" @click="$refs.fileInput.click()">
                <span v-if="selectedFiles.length === 0">
                  📁 Выберите файлы для загрузки
                </span>
                <span v-else>
                  Выбрано файлов: {{ selectedFiles.length }}
                </span>
              </div>
            </div>
            <div class="help-text">
              Пожалуйста, выберите хотя бы один файл для отправки решения.
            </div>
            <div v-if="selectedFiles.length > 0" class="selected-files">
              <div v-for="(file, index) in selectedFiles" :key="index" class="file-item">
                <span class="file-name">{{ file.name }}</span>
                <button type="button" @click="removeFile(index)" class="file-remove">×</button>
              </div>
            </div>
          </div>

          <div class="form-actions">
            <button
              type="submit"
              class="btn btn-success"
              :disabled="submitting || selectedFiles.length === 0"
            >
              {{ submitting ? 'Отправляем...' : selectedFiles.length === 0 ? 'Выберите файлы для отправки' : 'Отправить решение' }}
            </button>
          </div>
        </form>
      </div>

      <!-- Previous Submissions -->
      <div v-if="userSubmissions.length > 0" class="previous-submissions">
        <h2>Ваши предыдущие работы</h2>
        <div class="submissions-list">
          <div
            v-for="submission in userSubmissions"
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

            <p v-if="submission.comment" class="submission-comment">
              {{ submission.comment }}
            </p>

            <div class="submission-files">
              <strong>Файлы:</strong>
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
              <em>Ждём обратную связь...</em>
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

    <!-- Success Message -->
    <div v-if="successMessage" class="alert alert-success">
      {{ successMessage }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { apiService } from '../api'
import type { Problem, Submission } from '../types'

const route = useRoute()
const problemId = parseInt(route.params.id as string)

// Reactive state
const problem = ref<Problem | null>(null)
const userSubmissions = ref<Submission[]>([])
const loading = ref(true)
const submitting = ref(false)
const error = ref('')
const successMessage = ref('')
const validationError = ref('')
const submissionComment = ref('')
const selectedFiles = ref<File[]>([])
const fileInput = ref<HTMLInputElement>()

// Computed properties
const attemptCount = computed(() => userSubmissions.value.length)
const isAccepted = computed(() => userSubmissions.value.some(s => s.status.accepted))

// Methods
const loadProblem = async () => {
  try {
    loading.value = true
    const [problemData, allSubmissions] = await Promise.all([
      apiService.getProblem(problemId),
      apiService.getSubmissions()
    ])

    problem.value = problemData
    userSubmissions.value = allSubmissions.filter(s => s.problem === problemId)
  } catch (err) {
    error.value = 'Не удалось загрузить детали задачи'
    console.error('Error loading problem:', err)
  } finally {
    loading.value = false
  }
}

const handleFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files) {
    selectedFiles.value = Array.from(target.files)
    // Clear validation error when files are selected
    if (selectedFiles.value.length > 0) {
      validationError.value = ''
    }
  }
}

const removeFile = (index: number) => {
  selectedFiles.value.splice(index, 1)
}

const submitSolution = async () => {
  // Clear any previous validation errors
  validationError.value = ''

  if (selectedFiles.value.length === 0) {
    validationError.value = '⚠️ Пожалуйста, выберите хотя бы один файл для отправки решения'
    // Clear validation error after 8 seconds
    setTimeout(() => {
      validationError.value = ''
    }, 8000)
    return
  }

  try {
    submitting.value = true
    await apiService.createSubmission(problemId, submissionComment.value, selectedFiles.value)

    successMessage.value = 'Решение успешно отправлено!'
    submissionComment.value = ''
    selectedFiles.value = []
    if (fileInput.value) {
      fileInput.value.value = ''
    }

    // Reload submissions to show the new one
    await loadProblem()

    setTimeout(() => {
      successMessage.value = ''
    }, 5000)
  } catch (err) {
    validationError.value = 'Не удалось отправить решение. Попробуйте ещё раз.'
    // Clear validation error after 5 seconds
    setTimeout(() => {
      validationError.value = ''
    }, 5000)
    console.error('Error submitting solution:', err)
  } finally {
    submitting.value = false
  }
}

// Lifecycle
onMounted(() => {
  loadProblem()
})
</script>

<style scoped>
.problem-detail {
  max-width: 900px;
  margin: 0 auto;
}

.breadcrumb {
  margin-bottom: 1rem;
}

.breadcrumb-link {
  color: #007bff;
  text-decoration: none;
  font-size: 0.9rem;
}

.breadcrumb-link:hover {
  text-decoration: underline;
}

.problem-header {
  margin-bottom: 2rem;
}

.problem-header h1 {
  color: #2c3e50;
  margin: 0.5rem 0;
  font-size: 2rem;
}

.problem-meta {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.badge-accepted {
  background-color: #d4edda;
  color: #155724;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 500;
}

.attempts-badge {
  background-color: #e9ecef;
  color: #495057;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 500;
}

.problem-description {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  margin-bottom: 2rem;
}

.problem-description h2 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.description-content {
  color: #555;
  line-height: 1.8;
  font-size: 1rem;
  white-space: pre-wrap;
}

.submission-section {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  margin-bottom: 2rem;
}

.submission-section h2 {
  color: #2c3e50;
  margin-bottom: 1.5rem;
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
  border: 2px dashed #ddd;
  border-radius: 6px;
  text-align: center;
  cursor: pointer;
  transition: border-color 0.2s;
  color: #666;
}

.file-upload-label:hover {
  border-color: #007bff;
  color: #007bff;
}

.selected-files {
  margin-top: 1rem;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: #f8f9fa;
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.file-name {
  font-size: 0.9rem;
  color: #555;
}

.file-remove {
  background: none;
  border: none;
  color: #dc3545;
  font-size: 1.2rem;
  cursor: pointer;
  padding: 0;
  width: 24px;
  height: 24px;
  border-radius: 50%;
}

.file-remove:hover {
  background: #dc3545;
  color: white;
}

.form-actions {
  margin-top: 1.5rem;
}

.previous-submissions {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.previous-submissions h2 {
  color: #2c3e50;
  margin-bottom: 1.5rem;
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

.help-text {
  font-size: 0.85rem;
  color: #6c757d;
  margin-top: 0.5rem;
  font-style: italic;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn:disabled:hover {
  transform: none;
  box-shadow: none;
}

@media (max-width: 768px) {
  .problem-detail {
    padding: 0 1rem;
  }

  .problem-header h1 {
    font-size: 1.5rem;
  }

  .problem-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .submission-header {
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }

  .problem-description,
  .submission-section,
  .previous-submissions {
    padding: 1rem;
  }
}
</style>
