import { useState } from 'react';
import { Loader2, X } from 'lucide-react';

interface CreateProjectModalProps {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: (name: string, description: string) => Promise<void>;
}

export function CreateProjectModal({ isOpen, onClose, onSubmit }: CreateProjectModalProps) {
    const [name, setName] = useState('');
    const [description, setDescription] = useState('');
    const [isSubmitting, setIsSubmitting] = useState(false);

    if (!isOpen) return null;

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!name.trim()) return;

        setIsSubmitting(true);
        try {
            await onSubmit(name, description);
            // Reset form
            setName('');
            setDescription('');
            onClose();
        } catch (error) {
            console.error('Failed to create project:', error);
            // You might want to expose this error to the UI
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm">
            <div className="bg-[var(--color-bg-secondary)] border border-[var(--color-border)] rounded-2xl p-6 w-full max-w-md shadow-2xl relative">
                <button
                    onClick={onClose}
                    className="absolute top-4 right-4 text-[var(--color-text-muted)] hover:text-white transition-colors"
                >
                    <X className="w-5 h-5" />
                </button>

                <h2 className="text-xl font-bold text-white mb-6">Create New Project</h2>

                <form onSubmit={handleSubmit} className="space-y-4">
                    <div>
                        <label htmlFor="projectName" className="block text-sm font-medium text-[var(--color-text-secondary)] mb-1">
                            Project Name
                        </label>
                        <input
                            id="projectName"
                            type="text"
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                            placeholder="My Awesome Trip"
                            className="w-full bg-[var(--color-bg-tertiary)] border border-[var(--color-border)] rounded-lg px-4 py-2 text-white placeholder-[var(--color-text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent-primary)]/50"
                            autoFocus
                        />
                    </div>

                    <div>
                        <label htmlFor="projectDesc" className="block text-sm font-medium text-[var(--color-text-secondary)] mb-1">
                            Description (Optional)
                        </label>
                        <textarea
                            id="projectDesc"
                            value={description}
                            onChange={(e) => setDescription(e.target.value)}
                            placeholder="Highlights from summer vacation..."
                            rows={3}
                            className="w-full bg-[var(--color-bg-tertiary)] border border-[var(--color-border)] rounded-lg px-4 py-2 text-white placeholder-[var(--color-text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent-primary)]/50 resize-none"
                        />
                    </div>

                    <div className="flex justify-end gap-3 mt-6">
                        <button
                            type="button"
                            onClick={onClose}
                            className="px-4 py-2 text-sm font-medium text-[var(--color-text-secondary)] hover:text-white transition-colors"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            disabled={!name.trim() || isSubmitting}
                            className="flex items-center gap-2 px-4 py-2 bg-[var(--color-accent-primary)] text-white rounded-lg text-sm font-medium hover:bg-[var(--color-accent-primary)]/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {isSubmitting ? (
                                <>
                                    <Loader2 className="w-4 h-4 animate-spin" />
                                    Creating...
                                </>
                            ) : (
                                'Create Project'
                            )}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
}
