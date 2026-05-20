import { create } from "zustand";

interface UiState {
    isPinned: boolean;
    setIsPinned: (isPinned: boolean) => void;
}

const useUiStore = create<UiState>()((set) => ({
    isPinned: false,
    setIsPinned: (isPinned: boolean) => set({ isPinned }),
}));
export default useUiStore;
