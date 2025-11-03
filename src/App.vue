<template>
  <div class="app" :class="{ 'dark-mode': isDarkMode }">
    <div class="title-bar" :class="{ 'pinned': isPinned, 'shake-indicator': isShakingIndicator }" @mousedown="handleTitleBarMouseDown">
      <div class="title-bar-content">
        <div class="title-bar-left">
          <div class="logo">
            <svg width="32" height="32" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
              <circle cx="256" cy="256" r="240" fill="var(--theme-color, #2196f3)"/>
              <path d="M180 256 L220 316 L340 176" stroke="#ffffff" stroke-width="40" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <div class="app-title">
            <span class="app-name">T-DoList</span>
          </div>
          <div class="task-status" v-if="totalTasks === 0 || pendingTasks === 0">
            <div class="status-indicator"></div>
            <span class="status-text">暂无任务</span>
          </div>
          <div class="task-status" v-else-if="pendingTasks > 0">
            <div class="status-indicator"></div>
            <span class="status-text">未完成任务：{{ pendingTasks }}</span>
          </div>
        </div>
        
        <div class="title-bar-controls">
          <button 
            class="control-btn expand" 
            :class="{ expanded: isExpanded }"
            @click.stop="toggleExpanded" 
            @mouseenter="showTooltip($event, isExpanded ? '收缩' : '展开')"
            @mouseleave="hideTooltip"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="expand-icon">
              <polyline points="6,9 12,15 18,9"></polyline>
            </svg>
          </button>
          <button class="control-btn close" @click.stop="closeWindow" @mouseenter="showTooltip($event, '关闭')" @mouseleave="hideTooltip">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <div class="main-content" v-show="isExpanded">
      <div class="task-input-section">
        <div class="input-container">
          <input 
            v-model="newTask" 
            @keyup.enter="addTask"
            placeholder="添加新任务..."
            class="task-input"
          />
          <button @click.stop="addTask" class="add-btn" :disabled="!newTask.trim()">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"></line>
              <line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
          </button>
          <button 
            class="settings" 
            @click.stop="toggleSettings" 
            @mouseenter="showTooltip($event, '设置')"
            @mouseleave="hideTooltip"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="3"></circle>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1 1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
            </svg>
          </button>
        </div>
      </div>

      <div class="task-tabs">
        <div class="tab-buttons">
        <button 
          v-for="tab in tabs" 
          :key="tab.key"
          @click.stop="activeTab = tab.key"
          :class="['tab-btn', { active: activeTab === tab.key }]"
        >
          {{ tab.label }}
        </button>
        </div>
        
        <div class="import-export-buttons">
          <button 
            class="import-btn" 
            @click.stop="importData"
            @mouseenter="showTooltip($event, '导入数据')"
            @mouseleave="hideTooltip"
            title="导入数据"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <path d="M7 14l5-5 5 5"></path>
              <path d="M12 9V3"></path>
            </svg>
          </button>
          <button 
            class="export-btn" 
            @click.stop="exportData"
            @mouseenter="showTooltip($event, '导出数据')"
            @mouseleave="hideTooltip"
            title="导出数据"
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <path d="M7 10l5 5 5-5"></path>
              <path d="M12 15V3"></path>
            </svg>
          </button>
        </div>
      </div>

      <div class="task-list">
        <div 
          v-for="task in paginatedTasks" 
          :key="task.id"
          class="task-item"
          :class="{ 'completed-page': activeTab === 'completed' }"
          @click="showFullText(task)"
        >
          <div class="task-content">
            <input 
              type="checkbox" 
              v-model="task.completed"
              @change="handleTaskComplete(task)"
              @click.stop
              class="task-checkbox"
              :class="{ 'completed-page-checkbox': activeTab === 'completed' }"
              :style="getCheckboxStyle(task)"
            />
            <div class="task-info">
              <div class="task-text-container">
                <div class="task-text-row">
                  <span class="task-text" :class="{ completed: task.completed }">
                    {{ task.text }}
                  </span>
                </div>
              </div>
              <div class="task-times">
                <span class="task-time">
                  创建于 {{ formatDateTime(task.createdAt) }}
                </span>
                <span v-if="task.completed && task.completedAt" class="task-completed-time">
                  完成于 {{ formatDateTime(task.completedAt) }} (<span class="duration">耗时{{ calculateDuration(task.createdAt, task.completedAt) }}</span>)
                </span>
              </div>
            </div>
          </div>
          <button @click.stop="deleteTask(task.id)" class="delete-btn" @mouseenter="showTooltip($event, '删除任务')" @mouseleave="hideTooltip">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="3,6 5,6 21,6"></polyline>
              <path d="m19,6v14a2,2 0 0,1 -2,2H7a2,2 0 0,1 -2,-2V6m3,0V4a2,2 0 0,1 2,-2h4a2,2 0 0,1 2,2v2"></path>
              <line x1="10" y1="11" x2="10" y2="17"></line>
              <line x1="14" y1="11" x2="14" y2="17"></line>
            </svg>
          </button>
        </div>
        
        <div v-if="filteredTasks.length === 0" class="empty-state">
          <div class="empty-icon">
            <svg v-if="activeTab === 'completed'" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M9 12l2 2 4-4"></path>
              <path d="M21 12c.552 0 1-.448 1-1V5c0-.552-.448-1-1-1H3c-.552 0-1 .448-1 1v6c0 .552.448 1 1 1h18z"></path>
              <path d="M3 12h18v6c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1v-6z"></path>
            </svg>
            <svg v-else width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14,2 14,8 20,8"></polyline>
              <line x1="16" y1="13" x2="8" y2="13"></line>
              <line x1="16" y1="17" x2="8" y2="17"></line>
              <polyline points="10,9 9,9 8,9"></polyline>
            </svg>
          </div>
          <p>{{ activeTab === 'completed' ? '暂无已完成的任务' : '暂无任务' }}</p>
        </div>
      </div>

      <div v-if="totalPages > 1" class="pagination">
        <button 
          @click.stop="prevPage" 
          :disabled="currentPage === 1"
          class="pagination-btn prev"
          @mouseenter="showTooltip($event, '上一页')"
          @mouseleave="hideTooltip"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15,18 9,12 15,6"></polyline>
          </svg>
        </button>
        
        <div class="pagination-info">
          <span class="page-numbers">
            <button 
              v-for="page in visiblePages" 
              :key="page"
              @click.stop="page !== '...' && goToPage(page as number)"
              :class="['page-btn', { active: currentPage === page, ellipsis: page === '...' }]"
              :disabled="page === '...'"
            >
              {{ page }}
            </button>
          </span>
        </div>
        
        <button 
          @click.stop="nextPage" 
          :disabled="currentPage === totalPages"
          class="pagination-btn next"
          @mouseenter="showTooltip($event, '下一页')"
          @mouseleave="hideTooltip"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="9,18 15,12 9,6"></polyline>
          </svg>
        </button>
      </div>

      <div class="stats">
        <div class="stat-item">
          <span class="stat-number">{{ totalTasks }}</span>
          <span class="stat-label">总任务</span>
        </div>
        <div class="stat-item">
          <span class="stat-number">{{ completedTasks }}</span>
          <span class="stat-label">已完成</span>
        </div>
        <div class="stat-item">
          <span class="stat-number">{{ pendingTasks }}</span>
          <span class="stat-label">待完成</span>
        </div>
      </div>
    </div>

    <div v-if="showSettings" class="settings-menu">
      <div class="settings-content">
        <h3 class="settings-title">设置</h3>
        
        <div class="settings-section">
            <div class="settings-label-container">
          <label class="settings-label">主题色</label>
              <span class="settings-subtitle">选择应用的主题颜色</span>
            </div>
          <div class="color-picker">
                <button 
                  v-for="color in themeColors" 
                  :key="color.name"
                  @click.stop="setThemeColor(color.value)"
                  :class="['color-btn', { active: currentThemeColor === color.value }]"
                  :style="{ backgroundColor: isDarkMode ? color.darkValue : color.lightValue }"
                  @mouseenter="showTooltip($event, color.name)"
                  @mouseleave="hideTooltip"
                ></button>
          </div>
        </div>

        <div class="settings-section">
            <div class="settings-label-container">
          <label class="settings-label">外观模式</label>
              <span class="settings-subtitle">切换浅色/深色主题</span>
            </div>
          <div class="mode-toggle">
            <button 
              @click.stop="toggleDarkMode"
              :class="['mode-btn', { active: !isDarkMode }]"
            >
              浅色
            </button>
            <button 
              @click.stop="toggleDarkMode"
              :class="['mode-btn', { active: isDarkMode }]"
            >
              深色
            </button>
          </div>
        </div>

        <div class="settings-section">
          <div class="settings-label-container">
          <label class="settings-label">窗口层级</label>
            <span class="settings-subtitle">设置窗口显示层级</span>
          </div>
          <div class="window-level-toggle">
            <button 
              @click.stop="setWindowLevel('normal')"
              :class="['level-btn', { active: windowLevel === 'normal' }]"
            >
              普通层级
            </button>
            <button 
              @click.stop="setWindowLevel('top')"
              :class="['level-btn', { active: windowLevel === 'top' }]"
            >
              置于顶层
            </button>
          </div>
        </div>

          <div class="settings-section">
            <div class="settings-label-container">
            <label class="settings-label">窗口固定</label>
              <span class="settings-subtitle">固定窗口防止拖拽</span>
            </div>
            <div class="window-pin-toggle">
              <button
                @click.stop="togglePinned"
                :class="['level-btn', { active: !isPinned }]"
              >
                未固定
              </button>
              <button
                @click.stop="togglePinned"
                :class="['level-btn', { active: isPinned }]"
              >
                已固定
              </button>
            </div>
          </div>

        <div class="settings-section">
          <div class="version-info">
            <div class="version-item">
              <span class="version-label">version:</span>
              <span class="version-value">v{{ versionInfo.version }}</span>
              <span class="version-separator">|</span>
              <span class="version-label">author:</span>
              <span class="version-value">{{ versionInfo.author }}</span>
              <span class="version-separator">|</span>
              <a :href="versionInfo.repository.url" target="_blank" class="github-link" @click="openRepository">
                Source
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="showTextModal" class="text-modal-overlay" @click="closeTextModal">
      <div class="text-modal" @click.stop>
        <div class="text-modal-header">
          <h3 class="text-modal-title">任务详情</h3>
          <button @click="closeTextModal" class="text-modal-close" @mouseenter="showTooltip($event, '关闭')" @mouseleave="hideTooltip">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        <div class="text-modal-content">
          <div class="text-modal-text">{{ selectedTask?.text }}</div>
          <div class="text-modal-meta">
            <div class="text-modal-time">创建于 {{ selectedTask ? formatDateTime(selectedTask.createdAt) : '' }}</div>
            <div v-if="selectedTask?.completed && selectedTask?.completedAt" class="text-modal-completed-time">完成于 {{ formatDateTime(selectedTask.completedAt) }} (<span class="duration">耗时{{ calculateDuration(selectedTask.createdAt, selectedTask.completedAt) }}</span>)</div>
            <div class="text-modal-status">状态：{{ selectedTask?.completed ? '已完成' : '待完成' }}</div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="showDeleteModal" class="delete-modal-overlay" @click="closeDeleteModal">
      <div class="delete-modal" @click.stop>
        <div class="delete-modal-header">
          <div class="delete-modal-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="3,6 5,6 21,6"></polyline>
              <path d="m19,6v14a2,2 0 0,1 -2,2H7a2,2 0 0,1 -2,-2V6m3,0V4a2,2 0 0,1 2,-2h4a2,2 0 0,1 2,2v2"></path>
              <line x1="10" y1="11" x2="10" y2="17"></line>
              <line x1="14" y1="11" x2="14" y2="17"></line>
            </svg>
          </div>
          <h3 class="delete-modal-title">确认删除</h3>
        </div>
        <div class="delete-modal-content">
          <p class="delete-modal-message">您确定要删除这个任务吗？</p>
          <div class="delete-modal-task">
            <div class="delete-modal-task-label">任务内容：</div>
            <span class="delete-modal-task-text">{{ taskToDelete?.text }}</span>
            <div class="delete-modal-task-time">
              创建于 {{ taskToDelete ? formatDateTime(taskToDelete.createdAt) : '' }}
            </div>
          </div>
          <p class="delete-modal-warning">此操作无法撤销。</p>
        </div>
        <div class="delete-modal-actions">
          <button @click="closeDeleteModal" class="delete-modal-cancel">
            取消
          </button>
          <button @click="confirmDelete" class="delete-modal-confirm">
            确认删除
          </button>
        </div>
      </div>
    </div>

    <div v-if="showCompleteModal" class="complete-modal-overlay" @click="closeCompleteModal">
      <div class="complete-modal" @click.stop>
        <div class="complete-modal-header">
          <div class="complete-modal-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20,6 9,17 4,12"></polyline>
            </svg>
          </div>
          <h3 class="complete-modal-title">确认完成</h3>
        </div>
        <div class="complete-modal-content">
          <p class="complete-modal-message">您确定要将这个任务标记为已完成吗？</p>
          <div class="complete-modal-task">
            <div class="complete-modal-task-label">任务内容：</div>
            <span class="complete-modal-task-text">{{ taskToComplete?.text }}</span>
            <div class="complete-modal-task-time">
              创建于 {{ taskToComplete ? formatDateTime(taskToComplete.createdAt) : '' }}
            </div>
          </div>
          <p class="complete-modal-info">完成后任务将显示为已完成状态。</p>
        </div>
        <div class="complete-modal-actions">
          <button @click="closeCompleteModal" class="complete-modal-cancel">
            取消
          </button>
          <button @click="confirmComplete" class="complete-modal-confirm">
            确认完成
          </button>
        </div>
      </div>
    </div>

    <div v-if="showFireworks" class="fireworks-overlay">
      <div class="fireworks-container">
        <div class="firework" v-for="n in 16" :key="n" :style="{ '--delay': n * 0.08 + 's' }">
          <div class="firework-particle" v-for="i in 12" :key="i" :style="{ '--particle-delay': i * 0.03 + 's' }"></div>
        </div>
        <div class="celebration-text">🎉任务完成！🎉</div>
      </div>
    </div>

    <div 
      v-if="tooltip.show" 
      class="custom-tooltip"
      :class="[`tooltip-${tooltip.position}`]"
      :style="{ 
        left: tooltip.x + 'px', 
        top: tooltip.y + 'px',
        transform: getTooltipTransform(tooltip.position)
      }"
    >
      {{ tooltip.text }}
      <div class="tooltip-arrow" :class="tooltip.position"></div>
    </div>

    <div 
      v-if="showSuccessMessage"
      class="success-message"
      @click="showSuccessMessage = false"
    >
      <div class="success-icon">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20,6 9,17 4,12"></polyline>
        </svg>
      </div>
      <div class="success-text">{{ successMessage }}</div>
      <button class="close-btn" @click.stop="showSuccessMessage = false">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getVersionInfo, type VersionInfo } from './utils/version'

// 响应式数据
const newTask = ref('')
const isDarkMode = ref(false)
const activeTab = ref('all') // 当前活跃的标签页
const isExpanded = ref(false)
const maxVisibleTasks = ref(3) // 每页最大任务数量
const currentPage = ref(1) // 当前页码
const showSettings = ref(false) // 设置菜单显示状态
const currentThemeColor = ref('#1976d2') // 当前主题色
const windowLevel = ref('normal') // 窗口层级：top, normal
const isPinned = ref(false) // 窗口是否固定
const showTextModal = ref(false) // 文本详情弹窗显示状态
const selectedTask = ref<Task | null>(null) // 选中的任务
const showDeleteModal = ref(false) // 删除确认弹窗显示状态
const taskToDelete = ref<Task | null>(null) // 待删除的任务
const showCompleteModal = ref(false) // 任务完成确认弹窗显示状态
const taskToComplete = ref<Task | null>(null) // 待完成的任务
const showFireworks = ref(false) // 烟花庆祝动画显示状态
const isShakingIndicator = ref(false) // 小圆点抖动状态
const showSuccessMessage = ref(false) // 成功消息显示状态
const successMessage = ref('') // 成功消息内容

// Tooltip状态管理
const tooltip = ref({
  show: false,
  text: '',
  x: 0,
  y: 0,
  position: 'bottom' as 'top' | 'bottom' | 'left' | 'right'
})

// 版本信息
const versionInfo = ref<VersionInfo>(getVersionInfo())

// 任务数据
interface Task {
  id: number
  text: string
  completed: boolean
  createdAt: string
  completedAt?: string // 完成时间
}

const tasks = ref<Task[]>([])

// 标签页配置
const tabs = [
  { key: 'all', label: '任务清单' },
  { key: 'completed', label: '已完成' }
]

// 主题色配置 - 精选6种主题色
const themeColors = [
  // 绿色系
  { 
    name: 'Grass Green', 
    value: '#7cb342',
    lightValue: '#7cb342',
    darkValue: '#558b2f'
  },
  
  // 蓝色系
  { 
    name: 'Sky Blue', 
    value: '#2196f3',
    lightValue: '#2196f3',
    darkValue: '#1976d2'
  },
  { 
    name: 'Royal Blue', 
    value: '#3f51b5',
    lightValue: '#3f51b5',
    darkValue: '#303f9f'
  },
  
  // 紫色系
  { 
    name: 'Amethyst Purple', 
    value: '#9c27b0',
    lightValue: '#9c27b0',
    darkValue: '#7b1fa2'
  },
  
  // 红色系
  { 
    name: 'Scarlet Red', 
    value: '#f44336',
    lightValue: '#f44336',
    darkValue: '#d32f2f'
  },
  
  // 橙色系
  { 
    name: 'Orange', 
    value: '#ff9800',
    lightValue: '#ff9800',
    darkValue: '#f57c00'
  }
]

// 计算属性
const filteredTasks = computed(() => {
  let filtered: Task[]
  
  if (activeTab.value === 'completed') {
    // 已完成页面：只显示已完成的任务
    filtered = tasks.value.filter(task => task.completed)
  } else {
    // 任务清单页面：只显示未完成的任务
    filtered = tasks.value.filter(task => !task.completed)
  }
  
  // 排序：按创建时间排序（最新的在前）
  return filtered.sort((a, b) => {
    return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
  })
})

const totalTasks = computed(() => tasks.value.length)
const completedTasks = computed(() => tasks.value.filter(task => task.completed).length)
const pendingTasks = computed(() => tasks.value.filter(task => !task.completed).length)

// 分页相关计算属性
const totalPages = computed(() => {
  return Math.ceil(filteredTasks.value.length / maxVisibleTasks.value)
})

const paginatedTasks = computed(() => {
  const start = (currentPage.value - 1) * maxVisibleTasks.value
  const end = start + maxVisibleTasks.value
  return filteredTasks.value.slice(start, end)
})

// 智能分页：只显示相关的页码
const visiblePages = computed(() => {
  const total = totalPages.value
  const current = currentPage.value
  const pages: (number | string)[] = []
  
  if (total <= 7) {
    // 如果总页数少于等于7页，显示所有页码
    for (let i = 1; i <= total; i++) {
      pages.push(i)
    }
  } else {
    // 总页数超过7页，使用智能显示策略
    pages.push(1) // 总是显示第1页
    
    if (current > 4) {
      // 如果当前页离第1页较远，显示省略号
      pages.push('...')
    }
    
    // 计算需要显示的页码范围
    let startPage = Math.max(2, current - 2)
    let endPage = Math.min(total - 1, current + 2)
    
    // 确保在边界附近时显示更多页
    if (current <= 4) {
      endPage = Math.min(5, total - 1)
    } else if (current >= total - 3) {
      startPage = Math.max(2, total - 4)
    }
    
    // 添加连续页码
    for (let i = startPage; i <= endPage; i++) {
      pages.push(i)
    }
    
    if (current < total - 3) {
      // 如果当前页离最后一页较远，显示省略号
      pages.push('...')
    }
    
    if (total > 1) {
      pages.push(total) // 总是显示最后一页
    }
  }
  
  return pages
})

// 获取勾选按钮样式
const getCheckboxStyle = (task: Task) => {
  if (activeTab.value === 'completed' && task.completed) {
    return {
      backgroundColor: currentThemeColor.value,
      borderColor: currentThemeColor.value
    }
  }
  return {}
}

// 方法
const addTask = () => {
  if (newTask.value.trim()) {
    const now = new Date()
    const task: Task = {
      id: Date.now(),
      text: newTask.value.trim(),
      completed: false,
      createdAt: now.toISOString()
    }
    tasks.value.push(task)
    newTask.value = ''
    saveTasks()
  }
}

const deleteTask = (id: number) => {
  const task = tasks.value.find(t => t.id === id)
  if (task) {
    taskToDelete.value = task
    showDeleteModal.value = true
  }
}

const updateTask = () => {
  saveTasks()
}

// 处理任务完成勾选
const handleTaskComplete = (task: Task) => {
  console.log('handleTaskComplete called', task.completed, task)
  if (!task.completed) {
    // 如果取消勾选，清除完成时间并直接更新
    console.log('task unchecked, updating directly')
    task.completedAt = undefined
    updateTask()
  } else {
    // 如果勾选完成，先恢复未勾选状态，然后显示确认弹窗
    console.log('task checked, showing modal')
    task.completed = false
    taskToComplete.value = task
    showCompleteModal.value = true
  }
}

// 确认完成任务
const confirmComplete = () => {
  console.log('confirmComplete called', taskToComplete.value)
  if (taskToComplete.value) {
    // 找到原始任务并更新状态
    const originalTask = tasks.value.find(task => task.id === taskToComplete.value!.id)
    console.log('originalTask found', originalTask)
    if (originalTask) {
      originalTask.completed = true
      originalTask.completedAt = new Date().toISOString() // 记录完成时间
      console.log('task completed set to true', originalTask)
      saveTasks()
    }
    closeCompleteModal()
    
    // 显示烟花庆祝动画
    showFireworks.value = true
    setTimeout(() => {
      showFireworks.value = false
    }, 2000) // 3秒后自动关闭烟花动画
  }
}

// 关闭任务完成确认弹窗
const closeCompleteModal = () => {
  showCompleteModal.value = false
  taskToComplete.value = null
}

// 确认删除任务
const confirmDelete = () => {
  if (taskToDelete.value) {
    tasks.value = tasks.value.filter(task => task.id !== taskToDelete.value!.id)
    saveTasks()
    closeDeleteModal()
  }
}

// 关闭删除确认弹窗
const closeDeleteModal = () => {
  showDeleteModal.value = false
  taskToDelete.value = null
}

// 文本详情弹窗相关方法
const showFullText = (task: Task) => {
  selectedTask.value = task
  showTextModal.value = true
}

const closeTextModal = () => {
  showTextModal.value = false
  selectedTask.value = null
  hideTooltip()
}

// 时间格式化函数
const formatDateTime = (isoString: string) => {
  const date = new Date(isoString)
  
  // 始终显示具体的日期和时间
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  })
}

// 计算耗时时间
const calculateDuration = (createdAt: string, completedAt: string) => {
  const created = new Date(createdAt)
  const completed = new Date(completedAt)
  const diffMs = completed.getTime() - created.getTime()
  
  const diffMinutes = Math.floor(diffMs / (1000 * 60))
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))
  
  if (diffDays > 0) {
    return `${diffDays}天${diffHours % 24}小时`
  } else if (diffHours > 0) {
    return `${diffHours}小时${diffMinutes % 60}分钟`
  } else if (diffMinutes > 0) {
    return `${diffMinutes}分钟`
  } else {
    return '不到1分钟'
  }
}

// 分页控制方法
const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
  }
}

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++
  }
}

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--
  }
}

// 设置相关方法
const toggleSettings = () => {
  showSettings.value = !showSettings.value
}

const setThemeColor = (color: string) => {
  currentThemeColor.value = color
  localStorage.setItem('themeColor', color)
  
  // 应用主题色到 CSS 变量
  const root = document.documentElement
  const selectedColor = themeColors.find(c => c.value === color)
  if (selectedColor) {
    const colorValue = isDarkMode.value ? selectedColor.darkValue : selectedColor.lightValue
    root.style.setProperty('--theme-color', colorValue)
    root.style.setProperty('--theme-color-light', lightenColor(colorValue, 0.1))
    root.style.setProperty('--theme-color-dark', darkenColor(colorValue, 0.1))
  }
}

// 颜色工具函数
const lightenColor = (color: string, amount: number) => {
  const hex = color.replace('#', '')
  const r = parseInt(hex.substr(0, 2), 16)
  const g = parseInt(hex.substr(2, 2), 16)
  const b = parseInt(hex.substr(4, 2), 16)
  
  const newR = Math.min(255, Math.floor(r + (255 - r) * amount))
  const newG = Math.min(255, Math.floor(g + (255 - g) * amount))
  const newB = Math.min(255, Math.floor(b + (255 - b) * amount))
  
  return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`
}

const darkenColor = (color: string, amount: number) => {
  const hex = color.replace('#', '')
  const r = parseInt(hex.substr(0, 2), 16)
  const g = parseInt(hex.substr(2, 2), 16)
  const b = parseInt(hex.substr(4, 2), 16)
  
  const newR = Math.max(0, Math.floor(r * (1 - amount)))
  const newG = Math.max(0, Math.floor(g * (1 - amount)))
  const newB = Math.max(0, Math.floor(b * (1 - amount)))
  
  return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`
}

const setWindowLevel = async (level: 'top' | 'normal') => {
  windowLevel.value = level
  localStorage.setItem('windowLevel', level)
  
  try {
    if (level === 'top') {
      await invoke('set_window_always_on_top')
    } else {
      await invoke('set_window_normal_level')
    }
  } catch (error) {
    console.error('设置窗口层级失败:', error)
  }
}

const toggleDarkMode = () => {
  isDarkMode.value = !isDarkMode.value
  localStorage.setItem('isDarkMode', isDarkMode.value.toString())
  
  // 切换深浅模式时重新应用主题色
  if (currentThemeColor.value) {
    setThemeColor(currentThemeColor.value)
  }
}


const triggerShakeIndicator = () => {
  // 触发抖动动画
  isShakingIndicator.value = true
  
  // 动画结束后重置状态
  setTimeout(() => {
    isShakingIndicator.value = false
  }, 500) // 与CSS动画时长一致
}

// Tooltip显示和隐藏函数
const showTooltip = (event: MouseEvent, text: string) => {
  const rect = (event.target as HTMLElement).getBoundingClientRect()
  const viewportHeight = window.innerHeight
  const viewportWidth = window.innerWidth
  const tooltipHeight = 40 // 估算tooltip高度
  const tooltipWidth = text.length * 8 + 24 // 估算tooltip宽度
  
  // 检测是否在收缩状态（窗口高度较小）
  const isCollapsed = viewportHeight < 100
  
  // 在收缩状态下，使用更保守的定位策略
  let x, y, position: 'top' | 'bottom' | 'left' | 'right'
  
  if (isCollapsed) {
    // 收缩状态：优先显示在下方，如果下方空间不够则显示在上方，最后考虑左右侧
    const spaceBelow = viewportHeight - rect.bottom
    const spaceAbove = rect.top
    const spaceLeft = rect.left
    const spaceRight = viewportWidth - rect.right
    
    if (spaceBelow >= tooltipHeight + 10) {
      // 下方空间足够
      x = rect.left + rect.width / 2
      y = rect.bottom + 8
      position = 'bottom'
    } else if (spaceAbove >= tooltipHeight + 10) {
      // 上方空间足够
      x = rect.left + rect.width / 2
      y = rect.top - 8
      position = 'top'
    } else if (spaceLeft >= tooltipWidth + 10) {
      // 左侧空间足够
      x = rect.left - 8
      y = rect.top + rect.height / 2
      position = 'left'
    } else if (spaceRight >= tooltipWidth + 10) {
      // 右侧空间足够
      x = rect.right + 8
      y = rect.top + rect.height / 2
      position = 'right'
    } else {
      // 所有方向空间都不够，选择空间最大的一侧
      const spaces = [
        { space: spaceBelow, x: rect.left + rect.width / 2, y: rect.bottom + 8, pos: 'bottom' as const },
        { space: spaceAbove, x: rect.left + rect.width / 2, y: rect.top - 8, pos: 'top' as const },
        { space: spaceLeft, x: rect.left - 8, y: rect.top + rect.height / 2, pos: 'left' as const },
        { space: spaceRight, x: rect.right + 8, y: rect.top + rect.height / 2, pos: 'right' as const }
      ]
      
      const bestOption = spaces.reduce((max, current) => current.space > max.space ? current : max)
      x = bestOption.x
      y = bestOption.y
      position = bestOption.pos
    }
  } else {
    // 正常状态：智能选择位置
    const shouldShowTop = rect.bottom + tooltipHeight > viewportHeight && rect.top > tooltipHeight
    x = rect.left + rect.width / 2
    y = shouldShowTop ? rect.top - 8 : rect.bottom + 8
    position = shouldShowTop ? 'top' : 'bottom'
    
    // 计算x坐标，确保tooltip不会超出视口边界
    const minX = tooltipWidth / 2
    const maxX = viewportWidth - tooltipWidth / 2
    
    if (x < minX) {
      x = minX
    } else if (x > maxX) {
      x = maxX
    }
    
    // 如果tooltip会超出视口顶部，强制显示在下方
    if (shouldShowTop && y - tooltipHeight < 0) {
      y = rect.bottom + 8
      position = 'bottom'
    }
    
    // 如果tooltip会超出视口底部，强制显示在上方
    if (!shouldShowTop && y + tooltipHeight > viewportHeight) {
      y = rect.top - 8
      position = 'top'
    }
  }
  
  tooltip.value = {
    show: true,
    text,
    x,
    y,
    position
  }
}

const hideTooltip = () => {
  tooltip.value.show = false
}

// 显示成功消息
const showSuccess = (message: string) => {
  successMessage.value = message
  showSuccessMessage.value = true
  
  // 2秒后自动隐藏
  setTimeout(() => {
    showSuccessMessage.value = false
  }, 2000)
}

// 禁用开发者工具和右键菜单
const disableDevTools = () => {
  // 禁用右键菜单
  document.addEventListener('contextmenu', (e) => {
    e.preventDefault()
    return false
  })
  
  // 禁用F12和其他开发者工具快捷键
  document.addEventListener('keydown', (e) => {
    // F12
    if (e.key === 'F12') {
      e.preventDefault()
      return false
    }
    
    // Ctrl+Shift+I (开发者工具)
    if (e.ctrlKey && e.shiftKey && e.key === 'I') {
      e.preventDefault()
      return false
    }
    
    // Ctrl+Shift+J (控制台)
    if (e.ctrlKey && e.shiftKey && e.key === 'J') {
      e.preventDefault()
      return false
    }
    
    // Ctrl+U (查看源代码)
    if (e.ctrlKey && e.key === 'u') {
      e.preventDefault()
      return false
    }
    
    // Ctrl+Shift+C (检查元素)
    if (e.ctrlKey && e.shiftKey && e.key === 'C') {
      e.preventDefault()
      return false
    }
    
    // Ctrl+Shift+K (控制台)
    if (e.ctrlKey && e.shiftKey && e.key === 'K') {
      e.preventDefault()
      return false
    }
  })
  
  // 禁用选择文本时的右键菜单
  document.addEventListener('selectstart', (e) => {
    e.preventDefault()
    return false
  })
  
  // 禁用拖拽
  document.addEventListener('dragstart', (e) => {
    e.preventDefault()
    return false
  })
}

// 获取tooltip的transform样式
const getTooltipTransform = (position: string) => {
  switch (position) {
    case 'top':
      return 'translateX(-50%) translateY(-100%)'
    case 'bottom':
      return 'translateX(-50%)'
    case 'left':
      return 'translateX(-100%) translateY(-50%)'
    case 'right':
      return 'translateY(-50%)'
    default:
      return 'translateX(-50%)'
  }
}

const handleTitleBarMouseDown = async (e: MouseEvent) => {
  // 检查点击的是否是按钮
  const target = e.target as HTMLElement
  if (target.closest('.control-btn') || target.closest('.title-bar-controls')) {
    return
  }
  
  // 如果窗口已固定，触发抖动效果而不是拖拽
  if (isPinned.value) {
    triggerShakeIndicator()
    return;
  }
  
  // 使用Tauri内置的拖拽功能
  try {
    await invoke('start_drag')
  } catch (error) {
    console.error('拖拽窗口失败:', error)
  }
}

const toggleExpanded = async () => {
  isExpanded.value = !isExpanded.value
  
  try {
    await invoke('toggle_expanded', { isExpanded: isExpanded.value })
  } catch (error) {
    console.error('调整窗口大小失败:', error)
  }
}

const togglePinned = async () => {
  isPinned.value = !isPinned.value
  
  try {
    await invoke('set_window_pinned', { pinned: isPinned.value })
  } catch (error) {
    console.error('设置窗口固定状态失败:', error)
  }
}

const closeWindow = async () => {
  try {
    await invoke('toggle_window')
  } catch (error) {
    console.error('隐藏窗口失败:', error)
  }
}

const openRepository = () => {
  // 使用Tauri的shell API打开外部链接
  invoke('open_repository', { url: versionInfo.value.repository.url }).catch(console.error)
}

// 导入数据功能
const importData = async () => {
  try {
    // 使用HTML5文件输入API
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = '.json,application/json'
    input.style.display = 'none'
    document.body.appendChild(input)
    
    input.click()
    
    input.onchange = async (event) => {
      const file = (event.target as HTMLInputElement).files?.[0]
      if (file) {
        const text = await file.text()
        const importedData = JSON.parse(text)
        
        // 验证数据格式
        if (importedData.tasks && Array.isArray(importedData.tasks)) {
          // 直接覆盖现有任务数据
          tasks.value = importedData.tasks
          saveTasks()
          showSuccess(`成功导入 ${importedData.tasks.length} 个任务！`)
          
          // 如果导入的数据包含设置，则更新应用设置
          if (importedData.settings) {
            if (importedData.settings.isDarkMode !== undefined) {
              isDarkMode.value = importedData.settings.isDarkMode
              localStorage.setItem('isDarkMode', isDarkMode.value.toString())
            }
            
            if (importedData.settings.currentThemeColor) {
              currentThemeColor.value = importedData.settings.currentThemeColor
              localStorage.setItem('themeColor', currentThemeColor.value)
              setThemeColor(currentThemeColor.value)
            }
            
            if (importedData.settings.windowLevel) {
              windowLevel.value = importedData.settings.windowLevel
              localStorage.setItem('windowLevel', windowLevel.value)
              
              // 应用窗口层级设置
              if (windowLevel.value === 'top') {
                invoke('set_window_always_on_top').catch(console.error)
              } else {
                invoke('set_window_normal_level').catch(console.error)
              }
            }
          }
        } else {
          showSuccess('文件格式不正确，请选择有效的任务数据文件。')
        }
      }
      document.body.removeChild(input)
    }
    
    input.oncancel = () => {
      document.body.removeChild(input)
    }
  } catch (error) {
    console.error('导入数据失败:', error)
    showSuccess('导入数据失败，请检查文件格式是否正确。')
  }
}

// 导出数据功能
const exportData = async () => {
  try {
    // 创建文件内容
    const exportData = {
      tasks: tasks.value,
      settings: {
        isDarkMode: isDarkMode.value,
        currentThemeColor: currentThemeColor.value,
        windowLevel: windowLevel.value
      },
      exportDate: new Date().toISOString(),
      version: versionInfo.value.version
    }
    
    const dataStr = JSON.stringify(exportData, null, 2)
    const fileName = `T-DoList-${new Date().toISOString().split('T')[0]}.json`
    
    // 尝试使用File System Access API（现代浏览器支持）
    if ('showSaveFilePicker' in window) {
      try {
        const fileHandle = await (window as any).showSaveFilePicker({
          suggestedName: fileName,
          types: [{
            description: 'JSON files',
            accept: {
              'application/json': ['.json'],
            },
          }],
        })
        
        const writable = await fileHandle.createWritable()
        await writable.write(dataStr)
        await writable.close()
        
        showSuccess(`数据已成功导出到：${fileHandle.name}`)
        return
      } catch (error: any) {
        if (error.name !== 'AbortError') {
          console.log('File System Access API failed:', error)
        } else {
          // 用户取消了保存
          return
        }
      }
    }
    
    // 回退到HTML5下载
    const dataBlob = new Blob([dataStr], { type: 'application/json' })
    const url = URL.createObjectURL(dataBlob)
    const link = document.createElement('a')
    link.href = url
    link.download = fileName
    link.style.display = 'none'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
    
    showSuccess('数据已导出到下载文件夹！')
  } catch (error) {
    console.error('导出数据失败:', error)
    showSuccess('导出数据失败，请重试。')
  }
}

const saveTasks = () => {
  localStorage.setItem('tasks', JSON.stringify(tasks.value))
}

const loadTasks = () => {
  const savedTasks = localStorage.getItem('tasks')
  if (savedTasks) {
    const parsedTasks = JSON.parse(savedTasks)
    // 移除旧数据中的category字段，保持兼容性
    tasks.value = parsedTasks.map((task: any) => {
      const { category, ...taskWithoutCategory } = task
      return taskWithoutCategory
    })
  }
  
  const savedTheme = localStorage.getItem('isDarkMode')
  if (savedTheme) {
    isDarkMode.value = savedTheme === 'true'
  }
  
  const savedWindowLevel = localStorage.getItem('windowLevel')
  if (savedWindowLevel && (savedWindowLevel === 'top' || savedWindowLevel === 'normal')) {
    windowLevel.value = savedWindowLevel as 'top' | 'normal'
  }
}

// 生命周期
onMounted(() => {
  loadTasks()
  
  // 初始化主题色
  const savedThemeColor = localStorage.getItem('themeColor')
  if (savedThemeColor) {
    currentThemeColor.value = savedThemeColor
    setThemeColor(savedThemeColor)
  } else {
    // 默认使用 Sky Blue
    setThemeColor('#2196f3')
  }
  
  // 应用保存的窗口层级设置
  if (windowLevel.value === 'top') {
    invoke('set_window_always_on_top').catch(console.error)
  } else {
    invoke('set_window_normal_level').catch(console.error)
  }
  
  // 禁用开发者工具和右键菜单
  disableDevTools()
})
</script>

<style>
:root {
  --theme-color: #2196f3;
  --theme-color-light: #42a5f5;
  --theme-color-dark: #1976d2;
}

* {
  scrollbar-width: none;
  -ms-overflow-style: none;
}

*::-webkit-scrollbar {
  display: none;
}

::selection {
  background: var(--theme-color);
  color: white;
}

::-moz-selection {
  background: var(--theme-color);
  color: white;
}

.dark-mode ::selection {
  background: var(--theme-color);
  color: white;
}

.dark-mode ::-moz-selection {
  background: var(--theme-color);
  color: white;
}

.success-message {
  position: fixed;
  top: 80px;
  left: 50%;
  transform: translateX(-50%);
  background: white;
  border-radius: 12px;
  padding: 16px 20px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  border: 1px solid rgba(0, 0, 0, 0.08);
  display: flex;
  align-items: center;
  gap: 12px;
  z-index: 100000;
  max-width: 400px;
  min-width: 300px;
  animation: successSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(10px);
  cursor: pointer;
}

.success-icon {
  width: 24px;
  height: 24px;
  background: #4caf50;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.success-text {
  color: #333;
  font-size: 14px;
  font-weight: 500;
  line-height: 1.4;
  word-break: break-word;
  flex: 1;
}

.close-btn {
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: #666;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.close-btn:hover {
  background: rgba(0, 0, 0, 0.1);
  color: #333;
}

.dark-mode .success-message {
  background: rgba(30, 30, 30, 0.95);
  border-color: rgba(255, 255, 255, 0.1);
}

.dark-mode .success-text {
  color: #e0e0e0;
}

.dark-mode .close-btn {
  color: #999;
}

.dark-mode .close-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #e0e0e0;
}

@keyframes successSlideIn {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0) scale(1);
  }
}

button {
  pointer-events: auto !important;
  cursor: pointer !important;
}

* {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  -webkit-user-drag: none;
  -moz-user-drag: none;
}

input, textarea {
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  user-select: text;
}

html, body {
  overflow: hidden;
  margin: 0;
  padding: 0;
  height: 100%;
  pointer-events: auto;
}

#app {
  height: 100vh;
  overflow: hidden;
  pointer-events: auto;
}
</style>

<style scoped>
.app {
  width: 100%;
  height: 100vh;
  background: #ffffff;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', Roboto, sans-serif;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: all 0.3s ease;
  box-sizing: border-box;
  pointer-events: auto;
  border-radius: 12px;
  border: 1px solid rgba(0, 0, 0, 0.08);
}

.app.dark-mode {
  background: #1a1a1a !important;
  color: #ffffff;
  border: none !important;
  outline: none !important;
  box-shadow: none !important;
  overflow: hidden;
  border-radius: 0 !important;
}

/* Modrinth App风格标题栏 */
.title-bar {
  background: #ffffff;
  border-bottom: none;
  padding: 0;
  cursor: default;
  user-select: none;
  height: 56px;
  display: flex;
  align-items: center;
  position: relative;
  box-shadow: none;
  z-index: 1;
}

.dark-mode .title-bar {
  background: #1e1e1e;
  border-bottom: none;
  box-shadow: none;
}

/* 固定状态下的标题栏样式 - 左上角小圆点指示 */
.title-bar.pinned {
  position: relative;
}

.title-bar.pinned::before {
  content: '';
  position: absolute;
  top: 8px;
  left: 8px;
  width: 6px;
  height: 6px;
  background: var(--theme-color);
  border-radius: 50%;
  z-index: 10;
  box-shadow: 0 0 4px rgba(0, 0, 0, 0.2);
}

.dark-mode .title-bar.pinned::before {
  background: var(--theme-color);
  box-shadow: 0 0 4px rgba(255, 255, 255, 0.1);
}

/* 小圆点抖动动画 */
.title-bar.pinned.shake-indicator::before {
  animation: shakeDot 0.5s ease-in-out;
}

@keyframes shakeDot {
  0%, 100% { transform: translate(0, 0); }
  10% { transform: translate(-2px, -1px); }
  20% { transform: translate(2px, 1px); }
  30% { transform: translate(-1px, 2px); }
  40% { transform: translate(1px, -2px); }
  50% { transform: translate(-2px, 1px); }
  60% { transform: translate(2px, -1px); }
  70% { transform: translate(-1px, -2px); }
  80% { transform: translate(1px, 2px); }
  90% { transform: translate(-2px, -1px); }
}

.title-bar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  height: 100%;
  padding: 0 16px;
  position: relative;
  pointer-events: auto;
}

/* 左侧内容组 - Modrinth App风格 */
.title-bar-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.title-bar-left .task-status {
  margin-left: 24px;
}

.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 16px;
  background: transparent;
  overflow: hidden;
}

.logo img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.app-title {
  display: flex;
  align-items: center;
}

.app-name {
  font-size: 16px;
  font-weight: 500;
  color: #212121;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', Roboto, sans-serif;
}

.dark-mode .app-name {
  color: #ffffff;
}

/* 任务状态显示 - 参考图片样式 */
.task-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0;
  font-size: 13px;
  color: #757575;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', Roboto, sans-serif;
  font-weight: 500;
}

.dark-mode .task-status {
  color: #b0b0b0;
}

.status-indicator {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--theme-color);
  flex-shrink: 0;
  box-shadow: 0 0 8px var(--theme-color-light);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 8px var(--theme-color-light);
  }
  50% {
    box-shadow: 0 0 12px var(--theme-color);
  }
  100% {
    box-shadow: 0 0 8px var(--theme-color-light);
  }
}

.dark-mode .status-indicator {
  background: var(--theme-color);
  box-shadow: 0 0 8px var(--theme-color-light);
}

.status-text {
  font-weight: 400;
  white-space: nowrap;
}

/* 右侧控制按钮 - Modrinth App风格 */
.title-bar-controls {
  display: flex;
  gap: 8px;
  margin-left: auto;
  pointer-events: auto;
  z-index: 100;
}

.control-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  font-size: 14px;
  font-weight: 400;
  position: relative;
  background: rgba(0, 0, 0, 0.08);
  color: #757575;
  pointer-events: auto;
  z-index: 10;
}

.control-btn:hover {
  background: rgba(0, 0, 0, 0.12);
  transform: scale(1.05);
}

.control-btn:active {
  transform: scale(0.95);
}

.dark-mode .control-btn {
  background: rgba(255, 255, 255, 0.12);
  color: #b0b0b0;
}

.dark-mode .control-btn:hover {
  background: rgba(255, 255, 255, 0.16);
}

/* 展开按钮特殊样式 */
.expand {
  color: #757575;
  background: rgba(0, 0, 0, 0.08);
  border: none;
  outline: none;
  box-shadow: none;
}

.expand:hover {
  background: rgba(0, 0, 0, 0.12);
}

.expand:focus {
  background: rgba(0, 0, 0, 0.08);
  outline: none;
}

.dark-mode .expand {
  color: #b0b0b0;
  background: rgba(255, 255, 255, 0.12);
}

.dark-mode .expand:hover {
  background: rgba(255, 255, 255, 0.16);
}

.dark-mode .expand:focus {
  background: rgba(255, 255, 255, 0.12);
}

/* 关闭按钮特殊样式 */
.close {
  color: #757575;
  background: transparent;
}

.close:hover {
  background: #d32f2f;
  color: #ffffff;
}

.dark-mode .close {
  color: #b0b0b0;
  background: transparent;
}

.dark-mode .close:hover {
  background: #f44336;
  color: #ffffff;
}

/* SVG 图标样式 */
.control-btn svg {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.control-btn:hover svg {
  transform: scale(1.1) rotate(15deg);
}

.control-btn:not(:hover) svg {
  transform: scale(1) rotate(0deg);
}

/* 关闭按钮不旋转 */
.close:hover svg {
  transform: scale(1.1) rotate(0deg);
}

.close:not(:hover) svg {
  transform: scale(1) rotate(0deg);
}

/* 添加按钮和设置按钮旋转 */
.add-btn:hover svg {
  transform: scale(1.1) rotate(0deg);
}

.add-btn:not(:hover) svg {
  transform: scale(1) rotate(0deg);
}

/* 设置按钮图标不旋转，只缩放 */
.settings:hover svg {
  transform: scale(1.1) rotate(0deg);
}

.settings:not(:hover) svg {
  transform: scale(1) rotate(0deg);
}

/* 展开按钮图标旋转动画 */
.expand-icon {
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  transform-origin: center;
}

.expand.expanded .expand-icon {
  transform: rotate(180deg);
}

.main-content {
  flex: 1;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow: hidden;
  transition: all 0.3s ease;
  box-sizing: border-box;
  min-height: 0;
}

.task-input-section {
  margin-bottom: 0;
  flex-shrink: 0;
}

.input-container {
  display: flex;
  gap: 8px;
  align-items: center;
}

.task-input {
  flex: 1;
  padding: 12px 16px;
  border: 1px solid rgba(0, 0, 0, 0.12);
  border-radius: 24px;
  background: #ffffff;
  font-size: 14px;
  outline: none;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  font-family: 'Roboto', sans-serif;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.task-input::placeholder {
  color: #9e9e9e;
}

.dark-mode .task-input::placeholder {
  color: #b0b0b0;
}

.task-input:hover {
  border-color: rgba(0, 0, 0, 0.2);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08), 0 0 0 1px rgba(0, 0, 0, 0.05);
  transform: translateY(-1px);
}

.task-input:focus {
  border-color: var(--theme-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1), 0 0 0 1px var(--theme-color), 0 0 20px rgba(33, 150, 243, 0.15);
  transform: translateY(-1px);
}

.dark-mode .task-input {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
  color: #ffffff;
}

.dark-mode .task-input:hover {
  border-color: rgba(255, 255, 255, 0.2);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(255, 255, 255, 0.05);
}

.dark-mode .task-input:focus {
  border-color: var(--theme-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4), 0 0 0 1px var(--theme-color), 0 0 20px rgba(33, 150, 243, 0.2);
}

.add-btn {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 20px;
  background: var(--theme-color);
  color: white;
  font-size: 18px;
  font-weight: 400;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 4px rgba(0, 120, 212, 0.3);
  pointer-events: auto;
  z-index: 10;
}

.add-btn:hover:not(:disabled) {
  background: var(--theme-color-dark);
  transform: scale(1.05);
  box-shadow: 0 4px 8px rgba(0, 120, 212, 0.4);
}

.add-btn:active {
  transform: scale(0.95);
}

.add-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}

.dark-mode .add-btn {
  background: var(--theme-color);
  box-shadow: 0 2px 4px rgba(0, 120, 212, 0.3);
}

.dark-mode .add-btn:hover:not(:disabled) {
  background: var(--theme-color-dark);
  box-shadow: 0 4px 8px rgba(0, 120, 212, 0.4);
}

/* 设置按钮样式 - 与添加按钮相同大小 */
.settings {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 20px;
  background: rgba(0, 0, 0, 0.08);
  color: #757575;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  pointer-events: auto;
  z-index: 10;
}

.settings:hover {
  background: rgba(0, 0, 0, 0.12);
  transform: scale(1.05);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.settings:active {
  transform: scale(0.95);
}

.dark-mode .settings {
  color: #b0b0b0;
}

.dark-mode .settings:hover {
  background: rgba(255, 255, 255, 0.12);
}

.task-tabs {
  display: flex;
  gap: 4px;
  margin-bottom: 0;
  flex-shrink: 0;
  align-items: center;
  justify-content: space-between;
}

.tab-buttons {
  display: flex;
  gap: 4px;
}

.tab-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 20px;
  background: rgba(0, 0, 0, 0.08);
  color: #757575;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  font-family: 'Roboto', sans-serif;
}

.tab-btn:hover {
  background: rgba(0, 0, 0, 0.12);
}

.tab-btn.active {
  background: var(--theme-color);
  color: white;
  box-shadow: 0 2px 4px rgba(0, 120, 212, 0.3);
}

.dark-mode .tab-btn {
  background: rgba(255, 255, 255, 0.12);
  color: #b0b0b0;
}

.dark-mode .tab-btn:hover {
  background: rgba(255, 255, 255, 0.16);
}

.dark-mode .tab-btn.active {
  background: var(--theme-color);
  color: white;
  box-shadow: 0 2px 4px rgba(0, 120, 212, 0.3);
}

/* 导入导出按钮样式 */
.import-export-buttons {
  display: flex;
  gap: 8px;
  margin-left: auto;
}

.import-btn, .export-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 16px;
  background: rgba(0, 0, 0, 0.08);
  color: #757575;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  font-size: 14px;
  font-weight: 400;
  position: relative;
  pointer-events: auto;
  z-index: 10;
}

.import-btn:hover, .export-btn:hover {
  background: rgba(0, 0, 0, 0.12);
  transform: scale(1.05);
}

.import-btn:active, .export-btn:active {
  transform: scale(0.95);
}

.dark-mode .import-btn, .dark-mode .export-btn {
  background: rgba(255, 255, 255, 0.12);
  color: #b0b0b0;
}

.dark-mode .import-btn:hover, .dark-mode .export-btn:hover {
  background: rgba(255, 255, 255, 0.16);
}

.import-btn svg, .export-btn svg {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.import-btn:hover svg, .export-btn:hover svg {
  transform: scale(1.1);
}

.import-btn:not(:hover) svg, .export-btn:not(:hover) svg {
  transform: scale(1);
}

.task-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow: visible;
  min-height: 0;
  box-sizing: border-box;
  padding: 2px 0;
  justify-content: flex-start;
}

.task-item {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 10px 16px;
  background: #ffffff;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.12);
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  height: 50px;
  overflow: hidden;
  flex-shrink: 0;
  cursor: pointer;
}

.task-item:hover {
  background: #f5f5f5;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.dark-mode .task-item {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
}

.dark-mode .task-item:hover {
  background: #3e3e3e;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.task-item.completed {
  opacity: 0.6;
  background: #f8f9fa;
}

.dark-mode .task-item.completed {
  background: #1e1e1e;
}

/* 已完成页面的任务项保持正常样式 */
.task-item.completed-page {
  opacity: 1 !important;
  background: #ffffff !important;
}

.dark-mode .task-item.completed-page {
  background: #2d2d2d !important;
}

.task-content {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  flex: 1;
  min-width: 0;
  overflow: hidden;
}

.task-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  height: 100%;
  justify-content: space-between;
}

.task-checkbox {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  border: 2px solid #1976d2;
  cursor: pointer;
  background: #ffffff;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.task-checkbox:checked {
  background: #1976d2;
  border-color: #1976d2;
}

.dark-mode .task-checkbox {
  border-color: #2196f3;
  background: #2e2e2e;
}

.dark-mode .task-checkbox:checked {
  background: #2196f3;
  border-color: #2196f3;
}

/* 已完成页面的勾选按钮使用主题色 - 使用更强的选择器 */
.task-list .task-item.completed-page input.task-checkbox:checked {
  background: var(--theme-color) !important;
  border-color: var(--theme-color) !important;
  background-color: var(--theme-color) !important;
}

.dark-mode .task-list .task-item.completed-page input.task-checkbox:checked {
  background: var(--theme-color) !important;
  border-color: var(--theme-color) !important;
  background-color: var(--theme-color) !important;
}

/* 更具体的选择器确保优先级 */
.task-list .task-item.completed-page input[type="checkbox"]:checked {
  background: var(--theme-color) !important;
  border-color: var(--theme-color) !important;
}

.dark-mode .task-list .task-item.completed-page input[type="checkbox"]:checked {
  background: var(--theme-color) !important;
  border-color: var(--theme-color) !important;
}

/* 已完成页面勾选按钮的特殊样式 */
.completed-page-checkbox:checked {
  background: var(--theme-color) !important;
  border-color: var(--theme-color) !important;
}

.dark-mode .completed-page-checkbox:checked {
  background: var(--theme-color) !important;
  border-color: var(--theme-color) !important;
}

.task-text-container {
  display: flex;
  flex-direction: column;
  gap: 2px;
  width: 100%;
  overflow: hidden;
  flex: 1;
  min-height: 0;
}

.task-text-row {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  overflow: hidden;
}

.task-text {
  font-size: 14px;
  color: #212121;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  font-family: 'Roboto', sans-serif;
  font-weight: 400;
  line-height: 1.3;
  max-width: 100%;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  height: 1.3em;
  flex: 1;
  min-width: 0;
}

.task-text.completed {
  text-decoration: line-through;
  color: #757575;
}

.dark-mode .task-text {
  color: #ffffff;
}

.dark-mode .task-text.completed {
  color: #b0b0b0;
}

.task-times {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.task-time {
  font-size: 11px;
  color: #9e9e9e;
  font-weight: 400;
  font-family: 'Roboto', sans-serif;
  white-space: nowrap;
  flex-shrink: 0;
  height: 1.1em;
  line-height: 1.1em;
}

.task-completed-time {
  font-size: 10px;
  color: #4caf50;
  font-weight: 500;
  font-family: 'Roboto', sans-serif;
  white-space: nowrap;
  flex-shrink: 0;
  height: 1.0em;
  line-height: 1.0em;
}

.task-completed-time .duration {
  color: #ff9800;
  font-weight: 600;
}

.dark-mode .task-completed-time {
  color: #66bb6a;
}

.dark-mode .task-completed-time .duration {
  color: #ffb74d;
}


.delete-btn {
  background: none;
  border: none;
  font-size: 16px;
  cursor: pointer;
  padding: 6px;
  border-radius: 20px;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  opacity: 0.6;
  color: #d32f2f;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
}

.delete-btn:hover {
  opacity: 1;
  background: rgba(211, 47, 47, 0.12);
  transform: scale(1.1);
}

.dark-mode .delete-btn {
  color: #f44336;
}

.dark-mode .delete-btn:hover {
  background: rgba(244, 67, 54, 0.12);
}

.more-tasks {
  text-align: center;
  padding: 8px 16px;
  background: rgba(25, 118, 210, 0.08);
  border-radius: 16px;
  margin-top: 4px;
}

.more-tasks-text {
  font-size: 12px;
  color: #1976d2;
  font-weight: 500;
  font-family: 'Roboto', sans-serif;
}

.dark-mode .more-tasks {
  background: rgba(33, 150, 243, 0.12);
}

.dark-mode .more-tasks-text {
  color: #2196f3;
}

.empty-state {
  text-align: center;
  padding: 32px 20px;
  color: #757575;
}

.empty-icon {
  margin-bottom: 12px;
  color: #999999;
  display: flex;
  align-items: center;
  justify-content: center;
}

.dark-mode .empty-icon {
  color: #666666;
}

/* 分页控制样式 */
.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 8px 0;
  margin-top: 4px;
  flex-shrink: 0;
}

.pagination-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 16px;
  background: rgba(0, 0, 0, 0.08);
  color: #757575;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.pagination-btn:hover:not(:disabled) {
  background: rgba(0, 0, 0, 0.12);
  transform: scale(1.05);
}

.pagination-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
  transform: none;
}

.dark-mode .pagination-btn {
  background: rgba(255, 255, 255, 0.12);
  color: #b0b0b0;
}

.dark-mode .pagination-btn:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.16);
}

.pagination-info {
  display: flex;
  align-items: center;
}

.page-numbers {
  display: flex;
  gap: 4px;
}

.page-btn {
  min-width: 28px;
  height: 28px;
  border: none;
  border-radius: 14px;
  background: transparent;
  color: #757575;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  padding: 0 8px;
}

.page-btn:hover:not(:disabled) {
  background: rgba(0, 0, 0, 0.08);
}

.page-btn.active {
  background: var(--theme-color);
  color: white;
}

.page-btn.ellipsis {
  cursor: default;
  color: #9e9e9e;
  min-width: auto;
  padding: 0 4px;
}

.page-btn:disabled {
  cursor: not-allowed;
  opacity: 1;
}

.dark-mode .page-btn {
  color: #b0b0b0;
}

.dark-mode .page-btn:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.12);
}

.dark-mode .page-btn.active {
  background: var(--theme-color);
  color: white;
}

.dark-mode .page-btn.ellipsis {
  color: #666666;
}

.stats {
  display: flex;
  justify-content: space-around;
  padding: 12px 16px;
  background: #ffffff;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.12);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
}

.dark-mode .stats {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
}

.stat-item {
  text-align: center;
}

.stat-number {
  display: block;
  font-size: 16px;
  font-weight: 500;
  color: var(--theme-color);
  margin-bottom: 2px;
  font-family: 'Roboto', sans-serif;
}

.stat-label {
  font-size: 12px;
  color: #757575;
  font-weight: 400;
  font-family: 'Roboto', sans-serif;
}

.dark-mode .stat-number {
  color: var(--theme-color);
}

.dark-mode .stat-label {
  color: #b0b0b0;
}

/* 设置菜单样式 */
.settings-menu {
  position: absolute;
  top: 120px;
  right: 16px;
  width: 320px;
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  border: 1px solid rgba(0, 0, 0, 0.08);
  z-index: 1000;
  overflow: hidden;
}

.dark-mode .settings-menu {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.settings-content {
  padding: 16px 16px 0px 16px;
}

.settings-title {
  margin: 0 0 8px 0;
  font-size: 18px;
  font-weight: 600;
  color: #212121;
  text-align: center;
}

.dark-mode .settings-title {
  color: #ffffff;
}

.settings-section {
  margin-bottom: 8px;
}

.settings-label {
  display: block;
  margin-bottom: 0;
  font-size: 14px;
  font-weight: 500;
  color: #424242;
}

.settings-label-container {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.settings-subtitle {
  font-size: 11px;
  color: #999;
  font-weight: 400;
  opacity: 0.8;
}

.dark-mode .settings-label {
  color: #ffffff;
}

.dark-mode .settings-subtitle {
  color: #777;
}

/* 主题色选择器 */
.color-picker {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
  max-width: 100%;
}

.color-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 2px solid transparent;
}

.color-btn:hover {
  transform: scale(1.1);
}

.color-btn.active {
  border-color: #ffffff;
  box-shadow: 0 0 0 2px var(--theme-color);
}

/* 模式切换 */
.mode-toggle {
  display: flex;
  background: rgba(0, 0, 0, 0.08);
  border-radius: 8px;
  padding: 4px;
}

.dark-mode .mode-toggle {
  background: rgba(255, 255, 255, 0.12);
}

.mode-btn {
  flex: 1;
  padding: 8px 16px;
  border: none;
  background: transparent;
  color: #757575;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.mode-btn.active {
  background: var(--theme-color);
  color: white;
}

.dark-mode .mode-btn {
  color: #b0b0b0;
}

.dark-mode .mode-btn.active {
  background: var(--theme-color);
  color: white;
}

/* 窗口层级切换 */
.window-level-toggle {
  display: flex;
  background: rgba(0, 0, 0, 0.08);
  border-radius: 8px;
  padding: 4px;
}

.dark-mode .window-level-toggle {
  background: rgba(255, 255, 255, 0.12);
}

/* 窗口固定切换 */
.window-pin-toggle {
  display: flex;
  background: rgba(0, 0, 0, 0.08);
  border-radius: 8px;
  padding: 4px;
}

.dark-mode .window-pin-toggle {
  background: rgba(255, 255, 255, 0.12);
}

.level-btn {
  flex: 1;
  padding: 8px 12px;
  border: none;
  background: transparent;
  color: #757575;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.level-btn.active {
  background: var(--theme-color);
  color: white;
}

.dark-mode .level-btn {
  color: #b0b0b0;
}

.dark-mode .level-btn.active {
  background: var(--theme-color);
  color: white;
}


/* 版本信息 */
.version-info {
  display: flex;
  flex-direction: column;
}

.version-item {
  display: flex;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  flex-wrap: nowrap;
  gap: 4px;
  white-space: nowrap;
  overflow: hidden;
}

.version-item:last-child {
  border-bottom: none;
}

.dark-mode .version-item {
  border-bottom-color: rgba(255, 255, 255, 0.12);
}

.version-label {
  font-size: 14px;
  color: #757575;
  font-weight: 500;
  white-space: nowrap;
  flex-shrink: 0;
}

.dark-mode .version-label {
  color: #b0b0b0;
}

.version-value {
  font-size: 14px;
  color: #424242;
  font-weight: 400;
  white-space: nowrap;
  flex-shrink: 0;
}

.dark-mode .version-value {
  color: #ffffff;
}

.version-separator {
  font-size: 14px;
  color: #bdbdbd;
  margin: 0 4px;
  flex-shrink: 0;
}

.dark-mode .version-separator {
  color: rgba(255, 255, 255, 0.5);
}

.github-link {
  font-size: 14px;
  color: var(--theme-color);
  text-decoration: none;
  font-weight: 400;
  white-space: nowrap;
  flex-shrink: 0;
  cursor: pointer;
  transition: all 0.2s ease;
}

.github-link:hover {
  text-decoration: underline;
  color: var(--theme-color-dark);
}

.dark-mode .github-link {
  color: var(--theme-color);
}

.dark-mode .github-link:hover {
  color: var(--theme-color-light);
}

/* 文本详情弹窗样式 */
.text-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

.text-modal {
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(0, 0, 0, 0.1);
  max-width: 500px;
  width: 90%;
  max-height: 80vh;
  overflow: hidden;
  animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.dark-mode .text-modal {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(-20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.text-modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  background: #f8f9fa;
}

.dark-mode .text-modal-header {
  background: #1e1e1e;
  border-bottom-color: rgba(255, 255, 255, 0.12);
}

.text-modal-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #212121;
}

.dark-mode .text-modal-title {
  color: #ffffff;
}

.text-modal-close {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 16px;
  background: rgba(0, 0, 0, 0.08);
  color: #757575;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.text-modal-close:hover {
  background: rgba(0, 0, 0, 0.12);
  transform: scale(1.05);
}

.dark-mode .text-modal-close {
  background: rgba(255, 255, 255, 0.12);
  color: #b0b0b0;
}

.dark-mode .text-modal-close:hover {
  background: rgba(255, 255, 255, 0.16);
}

.text-modal-content {
  padding: 20px;
  max-height: 60vh;
  overflow-y: auto;
}

.text-modal-text {
  font-size: 16px;
  line-height: 1.6;
  color: #212121;
  margin-bottom: 20px;
  word-wrap: break-word;
  white-space: pre-wrap;
}

.dark-mode .text-modal-text {
  color: #ffffff;
}

.text-modal-meta {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-top: 16px;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.dark-mode .text-modal-meta {
  border-top-color: rgba(255, 255, 255, 0.12);
}

.text-modal-time,
.text-modal-completed-time,
.text-modal-status {
  font-size: 14px;
  color: #757575;
  font-weight: 500;
}

.text-modal-completed-time {
  color: #4caf50;
}

.text-modal-completed-time .duration {
  color: #ff9800;
  font-weight: 600;
}

.dark-mode .text-modal-time,
.dark-mode .text-modal-status {
  color: #b0b0b0;
}

.dark-mode .text-modal-completed-time {
  color: #66bb6a;
}

.dark-mode .text-modal-completed-time .duration {
  color: #ffb74d;
}

/* 删除确认弹窗样式 */
.delete-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

.delete-modal {
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(0, 0, 0, 0.1);
  max-width: 400px;
  width: 90%;
  overflow: hidden;
  animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.dark-mode .delete-modal {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
}

.delete-modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 16px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.dark-mode .delete-modal-header {
  border-bottom-color: rgba(255, 255, 255, 0.12);
}

.delete-modal-icon {
  width: 40px;
  height: 40px;
  border-radius: 20px;
  background: rgba(211, 47, 47, 0.1);
  color: #d32f2f;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.dark-mode .delete-modal-icon {
  background: rgba(244, 67, 54, 0.1);
  color: #f44336;
}

.delete-modal-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #212121;
}

.dark-mode .delete-modal-title {
  color: #ffffff;
}

.delete-modal-content {
  padding: 20px;
}

.delete-modal-message {
  margin: 0 0 16px 0;
  font-size: 15px;
  color: #424242;
  line-height: 1.5;
  font-weight: 500;
}

.dark-mode .delete-modal-message {
  color: #ffffff;
}

.delete-modal-task {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 16px;
  margin: 16px 0;
  border-left: 4px solid var(--theme-color);
  border: 1px solid rgba(0, 0, 0, 0.08);
  position: relative;
  overflow: hidden;
}

.dark-mode .delete-modal-task {
  background: #1e1e1e;
  border-left-color: var(--theme-color);
  border-color: rgba(255, 255, 255, 0.08);
}

.delete-modal-task-text {
  font-size: 15px;
  color: #212121;
  line-height: 1.6;
  word-wrap: break-word;
  white-space: pre-wrap;
  font-family: 'Roboto', sans-serif;
  font-weight: 400;
  margin: 0;
  position: relative;
  z-index: 1;
}

.delete-modal-task-time,
.complete-modal-task-time {
  font-size: 12px;
  color: #757575;
  font-weight: 500;
  margin-top: 8px;
  font-style: italic;
}

.dark-mode .delete-modal-task-time,
.dark-mode .complete-modal-task-time {
  color: #b0b0b0;
}

.delete-modal-task-label {
  font-size: 12px;
  color: #757575;
  font-weight: 600;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.dark-mode .delete-modal-task-label {
  color: #b0b0b0;
}

.delete-modal-task::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(0, 0, 0, 0.02) 0%, rgba(0, 0, 0, 0.05) 100%);
  pointer-events: none;
  z-index: 0;
}

.dark-mode .delete-modal-task::before {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.02) 0%, rgba(255, 255, 255, 0.05) 100%);
}

.delete-modal-task::after {
  content: '';
  position: absolute;
  top: 8px;
  right: 8px;
  width: 20px;
  height: 20px;
  background: var(--theme-color);
  border-radius: 50%;
  opacity: 0.1;
  z-index: 0;
}

.delete-modal-warning {
  margin: 12px 0 0 0;
  font-size: 12px;
  color: #d32f2f;
  font-weight: 500;
}

.dark-mode .delete-modal-warning {
  color: #f44336;
}

.delete-modal-actions {
  display: flex;
  gap: 12px;
  padding: 16px 20px 20px 20px;
  justify-content: flex-end;
}

.delete-modal-cancel {
  padding: 8px 16px;
  border: 1px solid rgba(0, 0, 0, 0.12);
  border-radius: 6px;
  background: #ffffff;
  color: #424242;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.delete-modal-cancel:hover {
  background: #f5f5f5;
  border-color: rgba(0, 0, 0, 0.2);
}

.dark-mode .delete-modal-cancel {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
  color: #ffffff;
}

.dark-mode .delete-modal-cancel:hover {
  background: #3e3e3e;
  border-color: rgba(255, 255, 255, 0.2);
}

.delete-modal-confirm {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  background: #d32f2f;
  color: #ffffff;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.delete-modal-confirm:hover {
  background: #b71c1c;
}

.dark-mode .delete-modal-confirm {
  background: #f44336;
}

.dark-mode .delete-modal-confirm:hover {
  background: #d32f2f;
}

/* 任务完成确认弹窗样式 */
.complete-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

.complete-modal {
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(0, 0, 0, 0.1);
  max-width: 400px;
  width: 90%;
  overflow: hidden;
  animation: modalSlideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.dark-mode .complete-modal {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
}

.complete-modal-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 16px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.dark-mode .complete-modal-header {
  border-bottom-color: rgba(255, 255, 255, 0.12);
}

.complete-modal-icon {
  width: 40px;
  height: 40px;
  border-radius: 20px;
  background: rgba(76, 175, 80, 0.1);
  color: #4caf50;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.dark-mode .complete-modal-icon {
  background: rgba(76, 175, 80, 0.15);
  color: #66bb6a;
}

.complete-modal-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #212121;
}

.dark-mode .complete-modal-title {
  color: #ffffff;
}

.complete-modal-content {
  padding: 20px;
}

.complete-modal-message {
  margin: 0 0 16px 0;
  font-size: 15px;
  color: #424242;
  line-height: 1.5;
  font-weight: 500;
}

.dark-mode .complete-modal-message {
  color: #ffffff;
}

.complete-modal-task {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 16px;
  margin: 16px 0;
  border-left: 4px solid var(--theme-color);
  border: 1px solid rgba(0, 0, 0, 0.08);
  position: relative;
  overflow: hidden;
}

.dark-mode .complete-modal-task {
  background: #1e1e1e;
  border-left-color: var(--theme-color);
  border-color: rgba(255, 255, 255, 0.08);
}

.complete-modal-task-text {
  font-size: 15px;
  color: #212121;
  line-height: 1.6;
  word-wrap: break-word;
  white-space: pre-wrap;
  font-family: 'Roboto', sans-serif;
  font-weight: 400;
  margin: 0;
  position: relative;
  z-index: 1;
}

.dark-mode .complete-modal-task-text {
  color: #ffffff;
}

.complete-modal-task-label {
  font-size: 12px;
  color: #757575;
  font-weight: 600;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.dark-mode .complete-modal-task-label {
  color: #b0b0b0;
}

.complete-modal-task::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(0, 0, 0, 0.02) 0%, rgba(0, 0, 0, 0.05) 100%);
  pointer-events: none;
  z-index: 0;
}

.dark-mode .complete-modal-task::before {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.02) 0%, rgba(255, 255, 255, 0.05) 100%);
}

.complete-modal-task::after {
  content: '';
  position: absolute;
  top: 8px;
  right: 8px;
  width: 20px;
  height: 20px;
  background: var(--theme-color);
  border-radius: 50%;
  opacity: 0.1;
  z-index: 0;
}

.complete-modal-info {
  margin: 16px 0 0 0;
  font-size: 12px;
  color: #4caf50;
  font-weight: 500;
}

.dark-mode .complete-modal-info {
  color: #66bb6a;
}

.complete-modal-actions {
  display: flex;
  gap: 12px;
  padding: 16px 20px 20px 20px;
  justify-content: flex-end;
}

.complete-modal-cancel {
  padding: 8px 16px;
  border: 1px solid rgba(0, 0, 0, 0.12);
  border-radius: 6px;
  background: #ffffff;
  color: #424242;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.complete-modal-cancel:hover {
  background: #f5f5f5;
  border-color: rgba(0, 0, 0, 0.2);
}

.dark-mode .complete-modal-cancel {
  background: #2e2e2e;
  border-color: rgba(255, 255, 255, 0.12);
  color: #ffffff;
}

.dark-mode .complete-modal-cancel:hover {
  background: #3e3e3e;
  border-color: rgba(255, 255, 255, 0.2);
}

.complete-modal-confirm {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  background: #4caf50;
  color: #ffffff;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.complete-modal-confirm:hover {
  background: #388e3c;
}

.dark-mode .complete-modal-confirm {
  background: #4caf50;
}

.dark-mode .complete-modal-confirm:hover {
  background: #388e3c;
}

/* 烟花庆祝动画样式 */
.fireworks-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 3000;
  pointer-events: none;
}

.fireworks-container {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.firework {
  position: absolute;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  animation: firework-explode 1.5s ease-out var(--delay) infinite;
}

.firework:nth-child(1) { top: 20%; left: 20%; }
.firework:nth-child(2) { top: 30%; left: 80%; }
.firework:nth-child(3) { top: 60%; left: 10%; }
.firework:nth-child(4) { top: 70%; left: 90%; }
.firework:nth-child(5) { top: 10%; left: 50%; }
.firework:nth-child(6) { top: 80%; left: 30%; }
.firework:nth-child(7) { top: 40%; left: 70%; }
.firework:nth-child(8) { top: 90%; left: 60%; }
.firework:nth-child(9) { top: 15%; left: 40%; }
.firework:nth-child(10) { top: 55%; left: 85%; }
.firework:nth-child(11) { top: 75%; left: 15%; }
.firework:nth-child(12) { top: 35%; left: 45%; }
.firework:nth-child(13) { top: 25%; left: 60%; }
.firework:nth-child(14) { top: 65%; left: 25%; }
.firework:nth-child(15) { top: 45%; left: 15%; }
.firework:nth-child(16) { top: 85%; left: 75%; }

.firework-particle {
  position: absolute;
  width: 6px;
  height: 6px;
  border-radius: 50%;
  animation: particle-fly 1.5s ease-out var(--particle-delay) infinite;
}

.firework:nth-child(odd) .firework-particle {
  background: #ff6b6b;
}

.firework:nth-child(even) .firework-particle {
  background: #4ecdc4;
}

.firework:nth-child(3n) .firework-particle {
  background: #45b7d1;
}

.firework:nth-child(4n) .firework-particle {
  background: #f9ca24;
}

.firework:nth-child(5n) .firework-particle {
  background: #f0932b;
}

.firework:nth-child(6n) .firework-particle {
  background: #eb4d4b;
}

.firework:nth-child(7n) .firework-particle {
  background: #6c5ce7;
}

.firework:nth-child(8n) .firework-particle {
  background: #a29bfe;
}

.celebration-text {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 48px;
  font-weight: bold;
  color: #fff;
  text-shadow: 0 0 20px rgba(255, 255, 255, 0.8);
  animation: celebration-bounce 2s ease-in-out infinite;
  z-index: 10;
  white-space: nowrap;
  text-align: center;
}

@keyframes firework-explode {
  0% {
    opacity: 0;
    transform: scale(0);
  }
  50% {
    opacity: 1;
    transform: scale(1);
  }
  100% {
    opacity: 0;
    transform: scale(0);
  }
}

@keyframes particle-fly {
  0% {
    opacity: 1;
    transform: translate(0, 0) scale(1);
  }
  100% {
    opacity: 0;
    transform: translate(var(--random-x, 100px), var(--random-y, 100px)) scale(0);
  }
}

@keyframes celebration-bounce {
  0%, 20%, 50%, 80%, 100% {
    transform: translate(-50%, -50%) scale(1);
  }
  40% {
    transform: translate(-50%, -50%) scale(1.1);
  }
  60% {
    transform: translate(-50%, -50%) scale(1.05);
  }
}

/* 为每个烟花粒子设置随机方向 */
.firework:nth-child(1) .firework-particle:nth-child(1) { --random-x: 80px; --random-y: -60px; }
.firework:nth-child(1) .firework-particle:nth-child(2) { --random-x: -70px; --random-y: -50px; }
.firework:nth-child(1) .firework-particle:nth-child(3) { --random-x: 60px; --random-y: 70px; }
.firework:nth-child(1) .firework-particle:nth-child(4) { --random-x: -80px; --random-y: 60px; }
.firework:nth-child(1) .firework-particle:nth-child(5) { --random-x: 90px; --random-y: -40px; }
.firework:nth-child(1) .firework-particle:nth-child(6) { --random-x: -60px; --random-y: -80px; }
.firework:nth-child(1) .firework-particle:nth-child(7) { --random-x: 70px; --random-y: 50px; }
.firework:nth-child(1) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 70px; }

.firework:nth-child(2) .firework-particle:nth-child(1) { --random-x: 70px; --random-y: -80px; }
.firework:nth-child(2) .firework-particle:nth-child(2) { --random-x: -60px; --random-y: -70px; }
.firework:nth-child(2) .firework-particle:nth-child(3) { --random-x: 80px; --random-y: 60px; }
.firework:nth-child(2) .firework-particle:nth-child(4) { --random-x: -70px; --random-y: 80px; }
.firework:nth-child(2) .firework-particle:nth-child(5) { --random-x: 60px; --random-y: -50px; }
.firework:nth-child(2) .firework-particle:nth-child(6) { --random-x: -80px; --random-y: -60px; }
.firework:nth-child(2) .firework-particle:nth-child(7) { --random-x: 50px; --random-y: 70px; }
.firework:nth-child(2) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 50px; }

.firework:nth-child(3) .firework-particle:nth-child(1) { --random-x: 90px; --random-y: -70px; }
.firework:nth-child(3) .firework-particle:nth-child(2) { --random-x: -80px; --random-y: -60px; }
.firework:nth-child(3) .firework-particle:nth-child(3) { --random-x: 70px; --random-y: 80px; }
.firework:nth-child(3) .firework-particle:nth-child(4) { --random-x: -60px; --random-y: 70px; }
.firework:nth-child(3) .firework-particle:nth-child(5) { --random-x: 80px; --random-y: -50px; }
.firework:nth-child(3) .firework-particle:nth-child(6) { --random-x: -70px; --random-y: -80px; }
.firework:nth-child(3) .firework-particle:nth-child(7) { --random-x: 60px; --random-y: 60px; }
.firework:nth-child(3) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 60px; }

.firework:nth-child(4) .firework-particle:nth-child(1) { --random-x: 60px; --random-y: -90px; }
.firework:nth-child(4) .firework-particle:nth-child(2) { --random-x: -70px; --random-y: -80px; }
.firework:nth-child(4) .firework-particle:nth-child(3) { --random-x: 80px; --random-y: 70px; }
.firework:nth-child(4) .firework-particle:nth-child(4) { --random-x: -60px; --random-y: 80px; }
.firework:nth-child(4) .firework-particle:nth-child(5) { --random-x: 70px; --random-y: -60px; }
.firework:nth-child(4) .firework-particle:nth-child(6) { --random-x: -80px; --random-y: -70px; }
.firework:nth-child(4) .firework-particle:nth-child(7) { --random-x: 50px; --random-y: 80px; }
.firework:nth-child(4) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 50px; }

.firework:nth-child(5) .firework-particle:nth-child(1) { --random-x: 80px; --random-y: -80px; }
.firework:nth-child(5) .firework-particle:nth-child(2) { --random-x: -70px; --random-y: -70px; }
.firework:nth-child(5) .firework-particle:nth-child(3) { --random-x: 70px; --random-y: 80px; }
.firework:nth-child(5) .firework-particle:nth-child(4) { --random-x: -80px; --random-y: 70px; }
.firework:nth-child(5) .firework-particle:nth-child(5) { --random-x: 60px; --random-y: -60px; }
.firework:nth-child(5) .firework-particle:nth-child(6) { --random-x: -60px; --random-y: -80px; }
.firework:nth-child(5) .firework-particle:nth-child(7) { --random-x: 80px; --random-y: 60px; }
.firework:nth-child(5) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 60px; }

.firework:nth-child(6) .firework-particle:nth-child(1) { --random-x: 70px; --random-y: -70px; }
.firework:nth-child(6) .firework-particle:nth-child(2) { --random-x: -80px; --random-y: -60px; }
.firework:nth-child(6) .firework-particle:nth-child(3) { --random-x: 60px; --random-y: 80px; }
.firework:nth-child(6) .firework-particle:nth-child(4) { --random-x: -70px; --random-y: 70px; }
.firework:nth-child(6) .firework-particle:nth-child(5) { --random-x: 80px; --random-y: -50px; }
.firework:nth-child(6) .firework-particle:nth-child(6) { --random-x: -60px; --random-y: -80px; }
.firework:nth-child(6) .firework-particle:nth-child(7) { --random-x: 70px; --random-y: 50px; }
.firework:nth-child(6) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 70px; }

.firework:nth-child(7) .firework-particle:nth-child(1) { --random-x: 90px; --random-y: -60px; }
.firework:nth-child(7) .firework-particle:nth-child(2) { --random-x: -60px; --random-y: -80px; }
.firework:nth-child(7) .firework-particle:nth-child(3) { --random-x: 80px; --random-y: 70px; }
.firework:nth-child(7) .firework-particle:nth-child(4) { --random-x: -70px; --random-y: 60px; }
.firework:nth-child(7) .firework-particle:nth-child(5) { --random-x: 60px; --random-y: -70px; }
.firework:nth-child(7) .firework-particle:nth-child(6) { --random-x: -80px; --random-y: -50px; }
.firework:nth-child(7) .firework-particle:nth-child(7) { --random-x: 70px; --random-y: 80px; }
.firework:nth-child(7) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 50px; }

.firework:nth-child(8) .firework-particle:nth-child(1) { --random-x: 60px; --random-y: -80px; }
.firework:nth-child(8) .firework-particle:nth-child(2) { --random-x: -70px; --random-y: -70px; }
.firework:nth-child(8) .firework-particle:nth-child(3) { --random-x: 80px; --random-y: 60px; }
.firework:nth-child(8) .firework-particle:nth-child(4) { --random-x: -60px; --random-y: 80px; }
.firework:nth-child(8) .firework-particle:nth-child(5) { --random-x: 70px; --random-y: -60px; }
.firework:nth-child(8) .firework-particle:nth-child(6) { --random-x: -80px; --random-y: -50px; }
.firework:nth-child(8) .firework-particle:nth-child(7) { --random-x: 50px; --random-y: 70px; }
.firework:nth-child(8) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 60px; }

.firework:nth-child(9) .firework-particle:nth-child(1) { --random-x: 80px; --random-y: -60px; }
.firework:nth-child(9) .firework-particle:nth-child(2) { --random-x: -60px; --random-y: -80px; }
.firework:nth-child(9) .firework-particle:nth-child(3) { --random-x: 70px; --random-y: 70px; }
.firework:nth-child(9) .firework-particle:nth-child(4) { --random-x: -80px; --random-y: 60px; }
.firework:nth-child(9) .firework-particle:nth-child(5) { --random-x: 60px; --random-y: -70px; }
.firework:nth-child(9) .firework-particle:nth-child(6) { --random-x: -70px; --random-y: -50px; }
.firework:nth-child(9) .firework-particle:nth-child(7) { --random-x: 80px; --random-y: 50px; }
.firework:nth-child(9) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 70px; }

.firework:nth-child(10) .firework-particle:nth-child(1) { --random-x: 70px; --random-y: -70px; }
.firework:nth-child(10) .firework-particle:nth-child(2) { --random-x: -80px; --random-y: -60px; }
.firework:nth-child(10) .firework-particle:nth-child(3) { --random-x: 60px; --random-y: 80px; }
.firework:nth-child(10) .firework-particle:nth-child(4) { --random-x: -70px; --random-y: 70px; }
.firework:nth-child(10) .firework-particle:nth-child(5) { --random-x: 80px; --random-y: -50px; }
.firework:nth-child(10) .firework-particle:nth-child(6) { --random-x: -60px; --random-y: -80px; }
.firework:nth-child(10) .firework-particle:nth-child(7) { --random-x: 70px; --random-y: 60px; }
.firework:nth-child(10) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 50px; }

.firework:nth-child(11) .firework-particle:nth-child(1) { --random-x: 90px; --random-y: -70px; }
.firework:nth-child(11) .firework-particle:nth-child(2) { --random-x: -60px; --random-y: -80px; }
.firework:nth-child(11) .firework-particle:nth-child(3) { --random-x: 80px; --random-y: 60px; }
.firework:nth-child(11) .firework-particle:nth-child(4) { --random-x: -70px; --random-y: 70px; }
.firework:nth-child(11) .firework-particle:nth-child(5) { --random-x: 60px; --random-y: -60px; }
.firework:nth-child(11) .firework-particle:nth-child(6) { --random-x: -80px; --random-y: -50px; }
.firework:nth-child(11) .firework-particle:nth-child(7) { --random-x: 70px; --random-y: 80px; }
.firework:nth-child(11) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 60px; }

.firework:nth-child(12) .firework-particle:nth-child(1) { --random-x: 60px; --random-y: -80px; }
.firework:nth-child(12) .firework-particle:nth-child(2) { --random-x: -70px; --random-y: -70px; }
.firework:nth-child(12) .firework-particle:nth-child(3) { --random-x: 80px; --random-y: 70px; }
.firework:nth-child(12) .firework-particle:nth-child(4) { --random-x: -60px; --random-y: 80px; }
.firework:nth-child(12) .firework-particle:nth-child(5) { --random-x: 70px; --random-y: -60px; }
.firework:nth-child(12) .firework-particle:nth-child(6) { --random-x: -80px; --random-y: -50px; }
.firework:nth-child(12) .firework-particle:nth-child(7) { --random-x: 50px; --random-y: 70px; }
.firework:nth-child(12) .firework-particle:nth-child(8) { --random-x: -90px; --random-y: 60px; }

/* 自定义Tooltip样式 */
.custom-tooltip {
  position: fixed;
  background: rgba(0, 0, 0, 0.9);
  color: #ffffff;
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  z-index: 99999;
  pointer-events: none;
  backdrop-filter: blur(8px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  animation: tooltipFadeIn 0.2s ease-out;
  margin-top: 8px;
  /* 确保tooltip不会被父容器的overflow影响 */
  transform-origin: center;
  will-change: transform, opacity;
}

.dark-mode .custom-tooltip {
  background: rgba(255, 255, 255, 0.95);
  color: #1a1a1a;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
}

.tooltip-arrow {
  position: absolute;
  width: 0;
  height: 0;
  border-style: solid;
}

.tooltip-arrow.top {
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  border-width: 6px 6px 0 6px;
  border-color: rgba(0, 0, 0, 0.9) transparent transparent transparent;
}

.dark-mode .tooltip-arrow.top {
  border-color: rgba(255, 255, 255, 0.95) transparent transparent transparent;
}

.tooltip-arrow.bottom {
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  border-width: 0 6px 6px 6px;
  border-color: transparent transparent rgba(0, 0, 0, 0.9) transparent;
}

.dark-mode .tooltip-arrow.bottom {
  border-color: transparent transparent rgba(255, 255, 255, 0.95) transparent;
}

.tooltip-arrow.left {
  left: 100%;
  top: 50%;
  transform: translateY(-50%);
  border-width: 6px 0 6px 6px;
  border-color: transparent transparent transparent rgba(0, 0, 0, 0.9);
}

.dark-mode .tooltip-arrow.left {
  border-color: transparent transparent transparent rgba(255, 255, 255, 0.95);
}

.tooltip-arrow.right {
  right: 100%;
  top: 50%;
  transform: translateY(-50%);
  border-width: 6px 6px 6px 0;
  border-color: transparent rgba(0, 0, 0, 0.9) transparent transparent;
}

.dark-mode .tooltip-arrow.right {
  border-color: transparent rgba(255, 255, 255, 0.95) transparent transparent;
}

@keyframes tooltipFadeIn {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}

</style>