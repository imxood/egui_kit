//! 图标常量
//!
//! 重导出 egui_phosphor 图标, 提供统一的图标访问和中文别名
//!
//! # 使用示例
//!
//! ```rust
//! use egui_kit::icons;
//!
//! // 使用原始名称
//! ui.label(format!("{} 设置", icons::GEAR));
//!
//! // 使用中文别名
//! ui.label(format!("{} 刷新", icons::REFRESH));
//! ```

// 重导出 egui_phosphor 图标模块
pub use egui_phosphor::regular::*;

// ===== 常用图标别名 =====

/// 刷新图标 (替代 🔄)
pub const REFRESH: &str = egui_phosphor::regular::ARROWS_CLOCKWISE;

/// 刷新图标 (圆形箭头)
pub const REFRESH_CIRCLE: &str = egui_phosphor::regular::ARROW_CLOCKWISE;

/// 文件图标 (替代 📄)
pub const FILE_DOC: &str = egui_phosphor::regular::FILE_TEXT;

/// 文件夹图标
pub const FOLDER: &str = egui_phosphor::regular::FOLDER;

/// 文件夹打开
pub const FOLDER_OPEN: &str = egui_phosphor::regular::FOLDER_OPEN;

/// 包/归档图标 (替代 📦)
pub const PACKAGE: &str = egui_phosphor::regular::PACKAGE;

/// 归档图标
pub const ARCHIVE: &str = egui_phosphor::regular::ARCHIVE;

/// 上传图标 (替代 📤)
pub const UPLOAD: &str = egui_phosphor::regular::UPLOAD_SIMPLE;

/// 下载图标 (替代 📥)
pub const DOWNLOAD: &str = egui_phosphor::regular::DOWNLOAD_SIMPLE;

/// 分享图标
pub const SHARE: &str = egui_phosphor::regular::SHARE_NETWORK;

/// 分享/发送
pub const SEND: &str = egui_phosphor::regular::PAPER_PLANE_RIGHT;

/// 收件箱/收到
pub const INBOX: &str = egui_phosphor::regular::TRAY;

/// 芯片图标 (替代 🔲)
pub const CHIP: &str = egui_phosphor::regular::CPU;

/// 方块图标
pub const SQUARE: &str = egui_phosphor::regular::SQUARE;

/// 设置图标
pub const SETTINGS: &str = egui_phosphor::regular::GEAR;

/// 用户图标
pub const USER: &str = egui_phosphor::regular::USER;

/// 用户组图标
pub const USERS: &str = egui_phosphor::regular::USERS;

/// 部门/组织图标
pub const DEPARTMENT: &str = egui_phosphor::regular::BUILDINGS;

/// 主页图标
pub const HOME: &str = egui_phosphor::regular::HOUSE;

/// 列表图标
pub const LIST: &str = egui_phosphor::regular::LIST;

/// 表格图标
pub const TABLE: &str = egui_phosphor::regular::TABLE;

/// 云图标
pub const CLOUD: &str = egui_phosphor::regular::CLOUD;

/// 锁图标
pub const LOCK: &str = egui_phosphor::regular::LOCK;

/// 解锁图标
pub const UNLOCK: &str = egui_phosphor::regular::LOCK_OPEN;

/// 盾牌/安全图标
pub const SHIELD: &str = egui_phosphor::regular::SHIELD;

/// 权限图标
pub const PERMISSION: &str = egui_phosphor::regular::SHIELD_CHECK;

/// 添加图标
pub const ADD: &str = egui_phosphor::regular::PLUS;

/// 添加 (圆形)
pub const ADD_CIRCLE: &str = egui_phosphor::regular::PLUS_CIRCLE;

/// 删除图标
pub const DELETE: &str = egui_phosphor::regular::TRASH;

/// 编辑图标
pub const EDIT: &str = egui_phosphor::regular::PENCIL;

/// 搜索图标
pub const SEARCH: &str = egui_phosphor::regular::MAGNIFYING_GLASS;

/// 过滤图标
pub const FILTER: &str = egui_phosphor::regular::FUNNEL;

/// 排序图标
pub const SORT: &str = egui_phosphor::regular::SORT_ASCENDING;

/// 复制图标
pub const COPY: &str = egui_phosphor::regular::COPY;

/// 粘贴图标
pub const PASTE: &str = egui_phosphor::regular::CLIPBOARD;

/// 剪切图标
pub const CUT: &str = egui_phosphor::regular::SCISSORS;

/// 撤销图标
pub const UNDO: &str = egui_phosphor::regular::ARROW_U_UP_LEFT;

/// 重做图标
pub const REDO: &str = egui_phosphor::regular::ARROW_U_UP_RIGHT;

/// 保存图标
pub const SAVE: &str = egui_phosphor::regular::FLOPPY_DISK;

/// 关闭图标
pub const CLOSE: &str = egui_phosphor::regular::X;

/// 关闭 (圆形)
pub const CLOSE_CIRCLE: &str = egui_phosphor::regular::X_CIRCLE;

/// 确认/勾选图标
pub const CHECK: &str = egui_phosphor::regular::CHECK;

/// 确认 (圆形)
pub const CHECK_CIRCLE: &str = egui_phosphor::regular::CHECK_CIRCLE;

/// 警告图标
pub const WARNING: &str = egui_phosphor::regular::WARNING;

/// 错误图标
pub const ERROR: &str = egui_phosphor::regular::X_CIRCLE;

/// 信息图标
pub const INFO: &str = egui_phosphor::regular::INFO;

/// 成功图标
pub const SUCCESS: &str = egui_phosphor::regular::CHECK_CIRCLE;

/// 问号图标
pub const QUESTION: &str = egui_phosphor::regular::QUESTION;

/// 展开图标
pub const EXPAND: &str = egui_phosphor::regular::CARET_DOWN;

/// 收起图标
pub const COLLAPSE: &str = egui_phosphor::regular::CARET_UP;

/// 向右箭头
pub const ARROW_RIGHT: &str = egui_phosphor::regular::ARROW_RIGHT;

/// 向左箭头
pub const ARROW_LEFT: &str = egui_phosphor::regular::ARROW_LEFT;

/// 向上箭头
pub const ARROW_UP: &str = egui_phosphor::regular::ARROW_UP;

/// 向下箭头
pub const ARROW_DOWN: &str = egui_phosphor::regular::ARROW_DOWN;

/// 更多选项 (三点)
pub const MORE: &str = egui_phosphor::regular::DOTS_THREE;

/// 菜单图标
pub const MENU: &str = egui_phosphor::regular::LIST;

/// 日志图标
pub const LOG: &str = egui_phosphor::regular::NOTE;

/// 审计图标
pub const AUDIT: &str = egui_phosphor::regular::CLIPBOARD_TEXT;

/// 测试图标
pub const TEST: &str = egui_phosphor::regular::TEST_TUBE;

/// 播放图标
pub const PLAY: &str = egui_phosphor::regular::PLAY;

/// 暂停图标
pub const PAUSE: &str = egui_phosphor::regular::PAUSE;

/// 停止图标
pub const STOP: &str = egui_phosphor::regular::STOP;

/// 链接图标
pub const LINK: &str = egui_phosphor::regular::LINK;

/// 外部链接
pub const EXTERNAL_LINK: &str = egui_phosphor::regular::ARROW_SQUARE_OUT;

/// 眼睛/查看图标
pub const VIEW: &str = egui_phosphor::regular::EYE;

/// 隐藏图标
pub const HIDE: &str = egui_phosphor::regular::EYE_SLASH;

/// 日历图标
pub const CALENDAR: &str = egui_phosphor::regular::CALENDAR;

/// 时钟图标
pub const CLOCK: &str = egui_phosphor::regular::CLOCK;

/// 代码图标
pub const CODE: &str = egui_phosphor::regular::CODE;

/// 终端图标
pub const TERMINAL: &str = egui_phosphor::regular::TERMINAL;

/// 数据库图标
pub const DATABASE: &str = egui_phosphor::regular::DATABASE;

/// 服务器图标
pub const SERVER: &str = egui_phosphor::regular::HARD_DRIVES;

/// 网络图标
pub const NETWORK: &str = egui_phosphor::regular::GLOBE;

/// 邮件图标
pub const MAIL: &str = egui_phosphor::regular::ENVELOPE;

/// 通知图标
pub const NOTIFICATION: &str = egui_phosphor::regular::BELL;

/// 星标图标
pub const STAR: &str = egui_phosphor::regular::STAR;

/// 收藏图标
pub const FAVORITE: &str = egui_phosphor::regular::HEART;

/// 标签图标
pub const TAG: &str = egui_phosphor::regular::TAG;

/// 评论图标
pub const COMMENT: &str = egui_phosphor::regular::CHAT;
