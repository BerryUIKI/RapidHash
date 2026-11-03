# 桌面端应用控件、对话框、菜单与无障碍标签

# 导航目的地
nav-calculate = 计算
    .description = 计算文件和目录的校验和
nav-verify = 验证
    .description = 对比预期摘要或校验清单验证文件
nav-generate = 生成
    .description = 为文件创建校验和清单
nav-settings = 设置
    .description = 配置算法、行为和外观偏好

# 计算视图
calculate-drop-zone = 将文件或文件夹拖拽至此处
    .hint = 或点击浏览
calculate-add-files = 添加文件
calculate-add-folder = 添加文件夹
calculate-clear-all = 清空全部
calculate-start = 计算校验和

# 验证视图
verify-target-input = 目标文件
    .placeholder = 选择或拖入要验证的文件
verify-digest-input = 预期校验和
    .placeholder = 在此粘贴预期的十六进制摘要
verify-manifest-input = 校验和清单
    .placeholder = 选择或拖入校验和清单文件（.sfv、.sha256 等）
verify-action-compare = 对比
verify-action-verify-manifest = 验证清单

# 结果表格列与表头
table-column-name = 文件名
table-column-path = 路径
table-column-size = 大小
table-column-algorithm = 算法
table-column-status = 状态
table-column-digest = 校验和
table-column-actions = 操作

# 结果表筛选与导出
calculate-filter-placeholder = 按文件名或路径筛选
calculate-filter-status = 状态筛选
calculate-filter-status-all = 全部状态
calculate-filter-status-ok = 正常
calculate-filter-status-mismatch = 不匹配
calculate-filter-status-error = 错误
calculate-showing-count = 显示 { $visible } / { $total } 项
calculate-show-more = 显示更多
calculate-filter-empty = 没有符合当前筛选条件的结果。
export-success = 已导出至 { $path }
export-failure = 导出失败：{ $error }

# 设置视图
settings-title = 应用设置
settings-algorithms = 已启用的算法
settings-appearance = 外观
settings-theme-auto = 跟随系统
settings-theme-light = 浅色
settings-theme-dark = 深色
settings-language = 语言
settings-reset = 恢复默认设置

# 无障碍标签
a11y-progress-label = 校验和计算进度
a11y-results-table = 校验和结果表格
a11y-copy-digest = 复制 { $path } 的摘要

# RapidCRC 特性操作
action-crc-into-filename = CRC 写入文件名
action-save-manifest = 导出校验清单
action-fold-all = 折叠哈希
action-expand-all = 展开哈希
action-hash-mode-auto = 自适应哈希
auto-calculate-label = 添加后自动计算

# CRC 写入文件名格式设置
settings-crc-format-title = CRC 写入文件名格式
settings-crc-format-pattern = 命名模板
settings-crc-format-casing = CRC 大小写
settings-crc-format-preview = 格式预览
settings-crc-uppercase = 大写 (如: 4E2A10FB)
settings-crc-lowercase = 小写 (如: 4e2a10fb)

# Windows 资源管理器右键菜单设置
settings-shell-integration = Windows 资源管理器右键菜单
settings-shell-integration-desc = 在 Windows 资源管理器文件与文件夹的右键菜单中添加 RapidHash 选项。
settings-shell-integration-win11-note = 提示：在 Windows 11 上，传统右键菜单项显示在“显示更多选项”(Shift+F10) 中。
settings-shell-integration-unsupported = 资源管理器集成仅在 Windows 系统可用。
shell-menu-entry-title = RapidHash



