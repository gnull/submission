<template>
  <div class="submission-detail">
    <!-- Loading State -->
    <div v-if="loading" class="loading">
      Загружаем работу...
    </div>

    <!-- Error State -->
    <div v-if="error" class="alert alert-error">
      {{ error }}
    </div>

    <!-- Submission Content -->
    <div v-if="submission && !loading" class="submission-content">
      <div class="submission-header">
        <div class="breadcrumb">
          <router-link to="/student" class="breadcrumb-link">← Назад к задачам</router-link>
        </div>
        <h1>Работа #{{ submission.id }}</h1>
        <div class="submission-meta">
          <span :class="submission.status.accepted ? 'status-accepted' : 'status-pending'">
            {{ submission.status.accepted ? '✅ Принято' : '❌ Не принято' }}
          </span>
        </div>
      </div>

      <!-- Problem Information -->
      <div class="problem-info card">
        <h2>Задача</h2>
        <div v-if="problemInfo" class="problem-details">
          <h3>{{ problemInfo.name }}</h3>
          <p>{{ problemInfo.desc }}</p>
          <router-link :to="`/problem/${problemInfo.id}`" class="btn btn-secondary">
            Открыть задачу
          </router-link>
        </div>
        <div v-else class="loading">
          Загружаем информацию о задаче...
        </div>
      </div>

      <!-- Submission Details -->
      <div class="submission-info card">
        <h2>Детали работы</h2>
        <div class="detail-item">
          <label>Комментарий:</label>
          <p>{{ submission.comment || 'Комментарий не добавлен' }}</p>
        </div>
      </div>

      <!-- Files Section -->
      <div class="files-section card">
        <h2>Отправленные файлы</h2>
        <div v-if="submission.files.length === 0" class="empty-state">
          <p>Файлы не были отправлены.</p>
        </div>
        <div v-else class="files-list">
          <div v-for="file in submission.files" :key="file.id" class="file-item">
            <div class="file-info">
              <h4>{{ file.name }}</h4>
              <div class="file-actions">
                <button @click="downloadFile(file)" class="btn btn-sm btn-primary">
                  📥 Скачать
                </button>
                <button @click="previewFile(file)" class="btn btn-sm btn-secondary">
                  👁️ Просмотр
                </button>
              </div>
            </div>

            <!-- File Preview -->
            <div v-if="previewingFile === file.id" class="file-preview">
              <div class="preview-header">
                <h5>Просмотр: {{ file.name }}</h5>
                <button @click="closePreview" class="btn-close">×</button>
              </div>
              <div class="preview-content">
                <div v-if="loadingPreview" class="loading">Загружаем просмотр...</div>
                <div v-else-if="previewError" class="preview-error">
                  Не удалось загрузить просмотр. Попробуйте скачать файл.
                </div>
                <pre v-else class="code-preview">{{ filePreviewContent }}</pre>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Feedback Section -->
      <div class="feedback-section card">
        <h2>Обратная связь</h2>
        <div v-if="submission.status.feedbacks.length === 0" class="empty-state">
          <p>Обратная связь пока не предоставлена. Ваша работа проверяется.</p>
        </div>
        <div v-else class="feedback-list">
          <div v-for="feedback in submission.status.feedbacks" :key="feedback.id" class="feedback-item">
            <div class="feedback-header">
              <h4>Отзыв #{{ feedback.id }}</h4>
              <span :class="feedback.grade === 1 ? 'feedback-accept' : 'feedback-reject'" class="feedback-grade">
                {{ feedback.grade === 1 ? '✅ Принято' : '❌ Отклонено' }}
              </span>
            </div>
            <div v-if="feedback.message" class="feedback-message">
              <p>{{ feedback.message }}</p>
            </div>
          </div>

          <!-- Overall Status -->
          <div class="overall-status">
            <h4>Overall Status:</h4>
            <p :class="submission.status.accepted ? 'status-success' : 'status-failure'">
              {{ submission.status.accepted
                ? 'Your submission has been accepted! The latest feedback from the reviewer was positive.'
                : 'Your submission has not been accepted. The latest feedback from the reviewer was negative.' }}
            </p>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="submission-actions">
        <router-link :to="`/problem/${submission.problem}`" class="btn btn-primary">
          Submit New Version
        </router-link>
        <button @click="goBack" class="btn btn-secondary">
          Go Back
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { apiService } from '../api'
import type { Submission, Problem, FileInfo } from '../types'

const route = useRoute()
const router = useRouter()
const submissionId = parseInt(route.params.id as string)

// Reactive state
const submission = ref<Submission | null>(null)
const problemInfo = ref<Problem | null>(null)
const loading = ref(true)
const error = ref('')
const previewingFile = ref<number | null>(null)
const filePreviewContent = ref('')
const loadingPreview = ref(false)
const previewError = ref(false)

// Methods
const loadSubmission = async () => {
  try {
    loading.value = true
    const submissionData = await apiService.getSubmission(submissionId)
    submission.value = submissionData

    // Load problem information
    const problemData = await apiService.getProblem(submissionData.problem)
    problemInfo.value = problemData

  } catch (err) {
    error.value = 'Не удалось загрузить детали работы'
    console.error('Error loading submission:', err)
  } finally {
    loading.value = false
  }
}

const downloadFile = async (file: FileInfo) => {
  try {
    const blob = await apiService.downloadFile(file.hash)
    const url = window.URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = file.name
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    window.URL.revokeObjectURL(url)
  } catch (err) {
    console.error('Error downloading file:', err)
    // Could show an error message to user
  }
}

const previewFile = async (file: FileInfo) => {
  try {
    previewingFile.value = file.id
    loadingPreview.value = true
    previewError.value = false

    const blob = await apiService.downloadFile(file.hash)
    const text = await blob.text()
    filePreviewContent.value = text

  } catch (err) {
    console.error('Error previewing file:', err)
    previewError.value = true
  } finally {
    loadingPreview.value = false
  }
}

const closePreview = () => {
  previewingFile.value = null
  filePreviewContent.value = ''
  previewError.value = false
}

const goBack = () => {
  router.go(-1)
}

// Lifecycle
onMounted(() => {
  loadSubmission()
})
</script>

<style scoped>
.submission-detail {
  max-width: 1000px;
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

.submission-header {
  margin-bottom: 2rem;
}

.submission-header h1 {
  color: #2c3e50;
  margin: 0.5rem 0;
  font-size: 2rem;
}

.submission-meta {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.status-accepted {
  background-color: #d4edda;
  color: #155724;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-weight: 500;
}

.status-pending {
  background-color: #f8d7da;
  color: #721c24;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-weight: 500;
}

.card {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  margin-bottom: 2rem;
}

.card h2 {
  color: #2c3e50;
  margin-bottom: 1.5rem;
  border-bottom: 2px solid #e9ecef;
  padding-bottom: 0.5rem;
}

.problem-details h3 {
  color: #2c3e50;
  margin-bottom: 1rem;
}

.problem-details p {
  color: #666;
  line-height: 1.6;
  margin-bottom: 1.5rem;
}

.detail-item {
  margin-bottom: 1rem;
}

.detail-item label {
  font-weight: 500;
  color: #2c3e50;
  display: block;
  margin-bottom: 0.5rem;
}

.detail-item p {
  color: #666;
  margin: 0;
  font-style: italic;
}

.files-list {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.file-item {
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 1.5rem;
  background: #f8f9fa;
}

.file-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.file-info h4 {
  color: #2c3e50;
  margin: 0;
}

.file-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-sm {
  padding: 0.25rem 0.5rem;
  font-size: 0.8rem;
}

.file-preview {
  margin-top: 1.5rem;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: #f8f9fa;
  border-bottom: 1px solid #ddd;
}

.preview-header h5 {
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

.preview-content {
  padding: 1rem;
  max-height: 400px;
  overflow-y: auto;
}

.code-preview {
  background: #f8f9fa;
  padding: 1rem;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.9rem;
  line-height: 1.4;
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.preview-error {
  color: #dc3545;
  text-align: center;
  padding: 2rem;
}

.feedback-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.feedback-item {
  padding: 1rem;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  background: #f8f9fa;
}

.feedback-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.feedback-header h4 {
  margin: 0;
  color: #2c3e50;
}

.feedback-grade {
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.8rem;
  font-weight: 500;
}

.feedback-accept {
  background-color: #d4edda;
  color: #155724;
}

.feedback-reject {
  background-color: #f8d7da;
  color: #721c24;
}

.feedback-message {
  margin-top: 0.75rem;
  padding-top: 0.75rem;
  border-top: 1px solid #e9ecef;
}

.feedback-message p {
  margin: 0;
  color: #495057;
  line-height: 1.5;
}

.overall-status {
  margin-top: 1.5rem;
  padding: 1.5rem;
  border-radius: 8px;
  border: 2px solid #e9ecef;
}

.overall-status h4 {
  margin: 0 0 1rem 0;
  color: #2c3e50;
}

.status-success {
  color: #155724;
  background-color: #d4edda;
  padding: 1rem;
  border-radius: 6px;
  margin: 0;
}

.status-failure {
  color: #721c24;
  background-color: #f8d7da;
  padding: 1rem;
  border-radius: 6px;
  margin: 0;
}

.submission-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 2px solid #e9ecef;
}

.empty-state {
  text-align: center;
  padding: 2rem;
  color: #6c757d;
}

@media (max-width: 768px) {
  .submission-detail {
    padding: 0 1rem;
  }

  .submission-header h1 {
    font-size: 1.5rem;
  }

  .file-info {
    flex-direction: column;
    gap: 1rem;
    align-items: flex-start;
  }

  .feedback-header {
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }

  .submission-actions {
    flex-direction: column;
  }

  .card {
    padding: 1rem;
  }
}
</style>
