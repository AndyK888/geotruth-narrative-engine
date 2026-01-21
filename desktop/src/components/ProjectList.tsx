import { Folder, Clock, Film, Plus } from 'lucide-react';

export interface Project {
  id: string;
  name: string;
  description?: string;
  video_count: number;
  updated_at: string;
  created_at: string;
}

interface ProjectListProps {
  projects: Project[];
  onCreateProject: () => void;
  onSelectProject: (project: Project) => void;
}

export function ProjectList({ projects, onCreateProject, onSelectProject }: ProjectListProps) {
  // Format date helper
  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diff = now.getTime() - date.getTime();

    // If less than 24 hours
    if (diff < 24 * 60 * 60 * 1000) {
      if (diff < 60 * 60 * 1000) {
        const mins = Math.max(1, Math.floor(diff / (60 * 1000)));
        return `${mins}m ago`;
      }
      const hours = Math.floor(diff / (60 * 60 * 1000));
      return `${hours}h ago`;
    }

    return date.toLocaleDateString();
  };

  return (
    <div className="w-full">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-bold flex items-center gap-2">
          <Folder className="w-5 h-5 text-[var(--color-accent-primary)]" />
          My Projects
        </h2>
        <button
          onClick={onCreateProject}
          className="flex items-center gap-2 px-3 py-1.5 bg-[var(--color-bg-tertiary)] hover:bg-[var(--color-accent-primary)]/20 text-sm font-medium rounded-lg transition-colors border border-[var(--color-border)] hover:border-[var(--color-accent-primary)]/50"
        >
          <Plus className="w-4 h-4" />
          New Project
        </button>
      </div>

      {projects.length === 0 ? (
        <div className="flex flex-col items-center justify-center p-12 border-2 border-dashed border-[var(--color-border)] rounded-xl bg-[var(--color-bg-secondary)]/30">
          <Folder className="w-12 h-12 text-[var(--color-text-muted)] mb-3 opacity-50" />
          <p className="text-[var(--color-text-secondary)] font-medium">No projects yet</p>
          <p className="text-sm text-[var(--color-text-muted)] mb-4">
            Create your first project to get started
          </p>
          <button
            onClick={onCreateProject}
            className="px-4 py-2 bg-[var(--color-accent-primary)] text-white rounded-lg text-sm font-medium hover:bg-[var(--color-accent-primary)]/90 transition-colors"
          >
            Create Project
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {projects.map((project) => (
            <div
              key={project.id}
              onClick={() => onSelectProject(project)}
              className="group relative bg-[var(--color-bg-secondary)] border border-[var(--color-border)] p-5 rounded-xl transition-all hover:border-[var(--color-accent-primary)]/50 hover:shadow-lg cursor-pointer"
            >
              <div className="flex justify-between items-start mb-3">
                <div className="p-2 bg-[var(--color-bg-tertiary)] rounded-lg group-hover:bg-[var(--color-accent-primary)]/10 transition-colors">
                  <Folder className="w-5 h-5 text-[var(--color-text-secondary)] group-hover:text-[var(--color-accent-primary)]" />
                </div>
                <span className="text-xs font-mono text-[var(--color-text-muted)] flex items-center gap-1">
                  <Clock className="w-3 h-3" />
                  {formatDate(project.updated_at)}
                </span>
              </div>

              <h3 className="font-semibold text-lg mb-1 truncate group-hover:text-[var(--color-accent-primary)] transition-colors">
                {project.name}
              </h3>

              <p className="text-sm text-[var(--color-text-muted)] mb-4 line-clamp-2 h-10">
                {project.description || 'No description'}
              </p>

              <div className="flex items-center gap-4 text-xs text-[var(--color-text-secondary)] pt-3 border-t border-[var(--color-border)]">
                <span className="flex items-center gap-1.5">
                  <Film className="w-3.5 h-3.5" />
                  {project.video_count} {project.video_count === 1 ? 'Video' : 'Videos'}
                </span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
