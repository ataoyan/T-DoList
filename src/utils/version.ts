// 版本信息管理
export interface VersionInfo {
  version: string;
  name: string;
  description: string;
  author: string;
  repository: {
    url: string;
    name: string;
  };
  build: {
    date: string;
    environment: string;
  };
}

// 从 package.json 读取版本信息
export function getVersionInfo(): VersionInfo {
  // 在构建时，这些信息会被注入
  return {
    version: import.meta.env.VITE_APP_VERSION || '0.2.0',
    name: import.meta.env.VITE_APP_NAME || 'T-DoList',
    description: import.meta.env.VITE_APP_DESCRIPTION || '轻量级桌面任务清单应用',
    author: import.meta.env.VITE_APP_AUTHOR || 'ATao',
    repository: {
      url: import.meta.env.VITE_APP_REPOSITORY_URL || 'https://github.com/ataoyan/T-DoList',
      name: 'T-DoList'
    },
    build: {
      date: import.meta.env.VITE_APP_BUILD_DATE || new Date().toISOString().split('T')[0],
      environment: import.meta.env.MODE || 'development'
    }
  };
}

// 格式化版本信息为显示文本
export function formatVersionInfo(info: VersionInfo): string {
  return `version:v${info.version} | author:${info.author} | Source`;
}
