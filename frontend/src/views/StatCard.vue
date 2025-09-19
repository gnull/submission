<template>
  <Card class="stat-card">
    <template #content>
      <div class="stat-item">
        <span class="stat-number">{{ number }}</span>
        <span class="stat-label">{{ label }}</span>
      </div>
    </template>
  </Card>
</template>

<script>
export default {
  name: 'StatCard',
  props: {
    number: {
      type: Number,
      required: true
    },
    label: {
      type: String,
      required: true
    }
  }
}
</script>

<style scoped>
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
</style>
