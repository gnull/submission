<template>
  <div class="home">
    <div class="hero">
      <h1>Добро пожаловать в Систему отправки заданий</h1>
      <p>Выберите свою роль для начала работы</p>
    </div>

    <div class="role-cards">
      <div class="role-card">
        <h2>🎓 Учитель</h2>
        <p>Создавайте задачи, проверяйте работы и давайте обратную связь ученикам.</p>
        <ul>
          <li>Создавать и управлять задачами</li>
          <li>Просматривать все работы</li>
          <li>Давать оценки (принять/отклонить)</li>
          <li>Следить за успехами учеников</li>
        </ul>
        <router-link to="/teacher" class="btn btn-primary">Войти как Учитель</router-link>
      </div>

      <div class="role-card">
        <h2>📚 Ученик</h2>
        <p>Просматривайте задачи, отправляйте решения и следите за своими успехами.</p>
        <ul>
          <li>Смотреть все доступные задачи</li>
          <li>Отправлять решения с кодом</li>
          <li>Отслеживать попытки сдачи</li>
          <li>Видеть принятые задачи выделенными</li>
        </ul>
        <router-link to="/student" class="btn btn-success">Войти как Ученик</router-link>
      </div>
    </div>

    <div class="features">
      <h2>Возможности системы</h2>
      <div class="feature-grid">
        <div class="feature-item">
          <h3>📝 Управление задачами</h3>
          <p>Учителя могут создавать задачи по программированию с подробным описанием.</p>
        </div>
        <div class="feature-item">
          <h3>📤 Отправка файлов</h3>
          <p>Ученики могут отправлять несколько файлов как решение с комментариями.</p>
        </div>
        <div class="feature-item">
          <h3>✅ Система обратной связи</h3>
          <p>Несколько отзывов на каждую работу с простой оценкой (принять/отклонить).</p>
        </div>
        <div class="feature-item">
          <h3>📊 Отслеживание прогресса</h3>
          <p>Просматривайте попытки сдачи и принятые задачи в одном месте.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// No additional logic needed for the home page
</script>

<style scoped>
.home {
  max-width: 1000px;
  margin: 0 auto;
}

.hero {
  text-align: center;
  padding: 3rem 0;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 12px;
  margin-bottom: 3rem;
}

.hero h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  font-weight: 700;
}

.hero p {
  font-size: 1.2rem;
  opacity: 0.9;
}

.role-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin-bottom: 3rem;
}

.role-card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  text-align: center;
  transition: transform 0.2s, box-shadow 0.2s;
}

.role-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.role-card h2 {
  font-size: 1.8rem;
  margin-bottom: 1rem;
  color: #333;
}

.role-card p {
  color: #666;
  margin-bottom: 1.5rem;
  line-height: 1.6;
}

.role-card ul {
  text-align: left;
  margin-bottom: 2rem;
  padding-left: 1.5rem;
}

.role-card li {
  margin-bottom: 0.5rem;
  color: #555;
}

.btn-primary {
  background-color: #667eea;
}

.btn-primary:hover {
  background-color: #5a6fd8;
}

.features {
  background: white;
  padding: 3rem;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.features h2 {
  text-align: center;
  margin-bottom: 2rem;
  font-size: 2rem;
  color: #333;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
}

.feature-item {
  padding: 1.5rem;
  border-radius: 8px;
  background: #f8f9fa;
  border-left: 4px solid #667eea;
}

.feature-item h3 {
  color: #333;
  margin-bottom: 1rem;
  font-size: 1.2rem;
}

.feature-item p {
  color: #666;
  line-height: 1.5;
}

@media (max-width: 768px) {
  .hero h1 {
    font-size: 2rem;
  }

  .hero {
    padding: 2rem 1rem;
  }

  .role-cards {
    grid-template-columns: 1fr;
  }

  .features {
    padding: 2rem 1rem;
  }

  .feature-grid {
    grid-template-columns: 1fr;
  }
}
</style>
