# CLI 消息、描述与诊断

# 命令描述
cli-description = RapidHash：快速、跨平台的校验和计算与比对工具
cli-hash-description = 计算文件或目录的密码学与非密码学校验和
cli-verify-description = 根据现有的校验和清单验证文件
cli-compare-description = 对比单个文件与预期的摘要
cli-algorithms-description = 列出可用的校验和算法及其状态
cli-completions-description = 生成终端补全脚本

# 参数与选项
cli-arg-path = 目标文件或目录路径
cli-arg-manifest = 校验和清单文件路径
cli-arg-digest = 预期的十六进制摘要
cli-arg-shell = 目标终端类型

cli-opt-algorithm = 要使用的校验和算法
cli-opt-format = 要使用的清单格式
cli-opt-output = 将输出写入指定的文件路径
cli-opt-json = 以结构化 JSON 格式输出结果
cli-opt-quiet = 隐藏提示消息与进度条
cli-opt-verbose = 启用详细的诊断输出
cli-opt-recursive = 递归遍历子目录
cli-opt-root = 设置清单路径的核准根目录

# 输出与进度摘要
cli-summary-verified = 已验证 { $total } 个文件：{ $matched } 个匹配，{ $mismatched } 个不匹配，{ $failed } 个失败
cli-item-progress = 正在处理 { $path } ({ $percent }%)
cli-compare-matched = { $path } 的校验和与预期摘要匹配。
cli-compare-mismatched = { $path } 的校验和与预期摘要不匹配。

# 错误与诊断
cli-error-file-not-found = 未找到文件：{ $path }
cli-error-permission-denied = 权限不足：{ $path }
cli-error-invalid-digest = 十六进制摘要格式无效：{ $digest }
cli-error-unsupported-algorithm = 不支持的算法：{ $algorithm }
cli-error-path-traversal = 路径尝试超出核准的根目录：{ $path }
cli-error-io = 读取 { $path } 时发生 I/O 错误：{ $error }
