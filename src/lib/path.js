import { detectOS } from './detectOS';

function getLastPath(path) {
    const separator = detectOS() === 'Windows' ? '\\' : '/';
    const segments = path.split(separator);
    return segments[segments.length - 1] ?? '';
}

export { getLastPath };
