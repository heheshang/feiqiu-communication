import { useToastStore } from '../store/toastStore';

/**
 * 错误代码枚举（与后端 ErrorCode 完全对应）
 */
export enum ErrorCode {
  Database = 0,
  Network = 1,
  Io = 2,
  Business = 3,
  Serialize = 4,
  Protocol = 5,
  NotFound = 6,
  AlreadyExists = 7,
  Validation = 8,
  Permission = 9,
}

/**
 * 前端错误结构（与后端 FrontendError 完全对应）
 */
export interface FrontendError {
  code: ErrorCode;
  message: string;
  details?: string;
}

/**
 * 错误代码到中文消息的映射
 */
const ERROR_MESSAGES: Record<ErrorCode, string> = {
  [ErrorCode.Database]: '数据库操作失败',
  [ErrorCode.Network]: '网络操作失败，请检查网络连接',
  [ErrorCode.Io]: '文件读写失败',
  [ErrorCode.Business]: '业务逻辑错误',
  [ErrorCode.Serialize]: '数据序列化失败',
  [ErrorCode.Protocol]: '协议解析错误',
  [ErrorCode.NotFound]: '资源不存在',
  [ErrorCode.AlreadyExists]: '资源已存在',
  [ErrorCode.Validation]: '输入数据验证失败',
  [ErrorCode.Permission]: '权限不足',
};

/**
 * 解析错误字符串为结构化错误对象
 *
 * @param errorString - 错误字符串（可能是 JSON 格式的结构化错误，也可能是普通字符串）
 * @returns 结构化的 FrontendError 对象
 *
 * @example
 * ```typescript
 * // 解析 JSON 格式错误
 * const error1 = parseError('{"code":0,"message":"数据库操作失败","details":"..."}');
 *
 * // 解析普通字符串错误（向后兼容）
 * const error2 = parseError('Something went wrong');
 * ```
 */
export function parseError(errorString: string): FrontendError {
  try {
    const parsed = JSON.parse(errorString);

    // 验证是否是有效的 FrontendError 结构
    if (
      typeof parsed === 'object' &&
      parsed !== null &&
      typeof parsed.code === 'number' &&
      typeof parsed.message === 'string'
    ) {
      return {
        code: parsed.code,
        message: parsed.message,
        details: parsed.details,
      };
    }

    // 不是有效的 FrontendError，作为普通错误处理
    return createFallbackError(errorString);
  } catch {
    // JSON 解析失败，作为普通错误处理
    return createFallbackError(errorString);
  }
}

/**
 * 创建后备错误对象（用于非结构化错误）
 */
function createFallbackError(message: string): FrontendError {
  return {
    code: ErrorCode.Business, // 默认为业务错误
    message: message || '未知错误',
    details: undefined,
  };
}

/**
 * 显示错误信息
 *
 * @param error - 错误对象
 *
 * @example
 * ```typescript
 * invoke('some_command', {})
 *   .catch((e: string) => {
 *     const error = parseError(e);
 *     showError(error);
 *   });
 * ```
 */
export function showError(error: FrontendError): void {
  const { addToast } = useToastStore.getState();
  addToast({
    message: error.message,
    type: 'error',
    duration: 5000,
  });
  console.error('[Error]', error.message);
  if (error.details) {
    console.error('[Details]', error.details);
  }
}

/**
 * 获取用户友好的错误消息
 *
 * @param error - 错误对象
 * @returns 用户友好的错误消息
 */
export function getErrorMessage(error: FrontendError): string {
  return error.message || ERROR_MESSAGES[error.code] || '操作失败';
}

/**
 * 检查错误是否匹配特定的错误代码
 *
 * @param error - 错误对象
 * @param code - 错误代码
 * @returns 如果匹配返回 true，否则返回 false
 *
 * @example
 * ```typescript
 * const error = parseError(errorString);
 * if (isErrorCode(error, ErrorCode.NotFound)) {
 *   // 处理 NotFound 错误
 * }
 * ```
 */
export function isErrorCode(error: FrontendError, code: ErrorCode): boolean {
  return error.code === code;
}
