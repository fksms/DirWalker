// TB/GB/MB/KBに変換
function toReadable(value) {
    if (value >= 1e12) {
        return [(value / 1e12).toFixed(1), 'TB'];
    } else if (value >= 1e9) {
        return [(value / 1e9).toFixed(1), 'GB'];
    } else if (value >= 1e6) {
        return [(value / 1e6).toFixed(1), 'MB'];
    } else if (value >= 1e3) {
        return [(value / 1e3).toFixed(1), 'KB'];
    } else {
        return [value.toFixed(1), 'B'];
    }
}

// 配列から文字列に変換
function formatReadableSize(value) {
    return toReadable(value).join(' ');
}

export { formatReadableSize, toReadable };
