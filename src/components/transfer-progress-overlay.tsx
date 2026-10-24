import { listen } from "@tauri-apps/api/event";
import { CheckCircle } from "lucide-react";
import { useEffect, useState } from "react";

interface TransferProgressPayload {
  current: number;
  total: number;
  operation: "upload" | "download";
}

interface TransferState {
  operation: "upload" | "download";
  current: number;
  total: number;
  done: boolean;
}

export function TransferProgressOverlay() {
  const [transfer, setTransfer] = useState<TransferState | null>(null);

  useEffect(() => {
    let hideTimeout: ReturnType<typeof setTimeout>;

    const unlistenProgress = listen<TransferProgressPayload>(
      "transfer:progress",
      (event) => {
        clearTimeout(hideTimeout);
        setTransfer({
          operation: event.payload.operation,
          current: event.payload.current,
          total: event.payload.total,
          done: false,
        });
      },
    );

    const unlistenDone = listen("transfer:done", () => {
      setTransfer((prev) =>
        prev ? { ...prev, current: prev.total, done: true } : null,
      );
      // Auto-dismiss after 1.5 s
      hideTimeout = setTimeout(() => {
        setTransfer(null);
      }, 1500);
    });

    return () => {
      clearTimeout(hideTimeout);
      void unlistenProgress.then((fn) => fn());
      void unlistenDone.then((fn) => fn());
    };
  }, []);

  if (!transfer) return null;

  const percentage =
    transfer.total > 0
      ? Math.round((transfer.current / transfer.total) * 100)
      : 0;

  const label =
    transfer.operation === "upload" ? "Uploading…" : "Downloading…";

  return (
    <div className="bg-background fixed right-4 bottom-4 z-50 w-72 rounded-lg border p-4 shadow-xl">
      <div className="mb-2 flex items-center justify-between gap-2">
        <span className="text-sm font-medium">
          {transfer.done ? (
            <span className="flex items-center gap-1.5 text-green-600">
              <CheckCircle className="size-4" />
              {transfer.operation === "upload"
                ? "Upload complete"
                : "Download complete"}
            </span>
          ) : (
            label
          )}
        </span>
        <span className="text-muted-foreground text-xs tabular-nums">
          {transfer.current} / {transfer.total}
        </span>
      </div>

      {/* Progress bar */}
      <div className="bg-secondary h-1.5 w-full overflow-hidden rounded-full">
        <div
          className="h-full rounded-full bg-blue-500 transition-all duration-300 ease-out"
          style={{ width: `${percentage}%` }}
        />
      </div>

      {!transfer.done && (
        <p className="text-muted-foreground mt-1.5 text-right text-xs">
          {percentage}%
        </p>
      )}
    </div>
  );
}
