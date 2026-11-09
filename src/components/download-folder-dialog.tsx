import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Archive, FolderDown } from "lucide-react";

interface DownloadFolderDialogProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  folderKey: string | null;
  onDownloadZip: (folderKey: string) => void;
  onDownloadIndividual: (folderKey: string) => void;
}

export function DownloadFolderDialog({
  isOpen,
  onOpenChange,
  folderKey,
  onDownloadZip,
  onDownloadIndividual,
}: DownloadFolderDialogProps) {
  if (!folderKey) return null;

  const displayName = folderKey.replace(/\/$/, "").split("/").pop() || folderKey;

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Download folder</DialogTitle>
          <DialogDescription>
            Choose how you want to download &ldquo;{displayName}&rdquo; to your computer.
          </DialogDescription>
        </DialogHeader>

        <div className="grid grid-cols-1 gap-3 py-3">
          <Button
            variant="outline"
            className="flex h-auto flex-col items-start gap-1 p-4 text-left hover:border-primary hover:bg-accent"
            onClick={() => {
              onOpenChange(false);
              onDownloadIndividual(folderKey);
            }}
          >
            <div className="flex items-center gap-2 font-semibold">
              <FolderDown className="h-4 w-4 text-primary" />
              <span>Individual files</span>
            </div>
            <span className="text-xs text-muted-foreground font-normal">
              Download all files preserving the folder structure.
            </span>
          </Button>

          <Button
            variant="outline"
            className="flex h-auto flex-col items-start gap-1 p-4 text-left hover:border-primary hover:bg-accent"
            onClick={() => {
              onOpenChange(false);
              onDownloadZip(folderKey);
            }}
          >
            <div className="flex items-center gap-2 font-semibold">
              <Archive className="h-4 w-4 text-primary" />
              <span>ZIP archive</span>
            </div>
            <span className="text-xs text-muted-foreground font-normal">
              Bundle all files into a single compressed .zip file.
            </span>
          </Button>
        </div>

        <DialogFooter>
          <DialogClose asChild>
            <Button variant="ghost">Cancel</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
