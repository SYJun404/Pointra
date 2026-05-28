/** 将 KeyboardEvent 转成规范 keys 数组 */
export function eventToKeys(e: React.KeyboardEvent): string[] {
    const parts: string[] = [];
    if (e.ctrlKey) parts.push("Ctrl");
    if (e.metaKey) parts.push("Cmd");
    if (e.altKey) parts.push("Alt");
    if (e.shiftKey) parts.push("Shift");

    const key =
        e.key === "Control" ||
        e.key === "Meta" ||
        e.key === "Alt" ||
        e.key === "Shift"
            ? null
            : e.key.length === 1
              ? e.key.toUpperCase()
              : e.key;

    if (key) parts.push(key);
    return parts;
}

/** 检测快捷键冲突，返回存在冲突的 id 集合 */
export function getConflictIds(
    shortcuts: { id: string; keys: string[] }[],
): Set<string> {
    const map = new Map<string, string[]>();
    for (const s of shortcuts) {
        if (s.keys.length === 0) continue;
        const key = s.keys.join("+");
        if (!map.has(key)) map.set(key, []);
        map.get(key)!.push(s.id);
    }
    const conflictSet = new Set<string>();
    for (const ids of map.values()) {
        if (ids.length > 1) ids.forEach((id) => conflictSet.add(id));
    }
    return conflictSet;
}
/**防止 "Ctrl + A" vs "A + Ctrl" 这种问题 */
export function normalizeKeys(keys: string[]) {
    const modifiers = ["Ctrl", "Cmd", "Alt", "Shift"];

    const mods = keys.filter((k) => modifiers.includes(k));
    const others = keys.filter((k) => !modifiers.includes(k));

    return [...mods.sort(), ...others.sort()];
}
