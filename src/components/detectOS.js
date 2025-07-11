// OSを判定
//
// returnは（"Mac" || "Windows" || "Linux" || null）
function detectOS() {
    const ua = navigator.userAgent;

    if (!ua) {
        return null;
    }

    if (ua.indexOf('Mac') > -1) {
        return 'Mac';
    } else if (ua.indexOf('Windows') > -1) {
        return 'Windows';
    } else if (ua.indexOf('Linux') > -1) {
        return 'Linux';
    } else {
        return null;
    }
}

// 外部に公開
export { detectOS };
