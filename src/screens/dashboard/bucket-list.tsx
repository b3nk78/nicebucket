import { BucketInfo } from "@/bindings";
import { FileTree } from "@/components/file-tree";
import { FileTreeSkeleton } from "@/components/file-tree-skeleton";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuShortcut,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { copyToClipboard } from "@/lib/actions";
import { useCommands } from "@/lib/use-commands";
import { cn } from "@/lib/utils";
import { skipToken, useQuery } from "@tanstack/react-query";
import { EllipsisVertical, Folder, RotateCw } from "lucide-react";
import { createPortal } from "react-dom";
import { toast } from "sonner";
import { useDashboardContext } from "./use-dashboard-context";

interface BucketListProps {
  headerPortalRef: React.RefObject<HTMLDivElement | null>;
}

export function BucketList({ headerPortalRef }: BucketListProps) {
  const { commands } = useCommands();
  const { connection, setSelectedBucket, searchPhrase } = useDashboardContext();

  const { data, isPending, isFetching, isError, error, refetch } = useQuery({
    queryKey: ["listBuckets", connection?.id],

    queryFn: connection
      ? () => {
        return commands.listBuckets(connection);
      }
      : skipToken,

    select: (buckets: BucketInfo[]) => {
      return buckets
        .filter((bucket) => {
          return (
            !searchPhrase ||
            bucket.name.toLowerCase().includes(searchPhrase.toLowerCase())
          );
        })
        .map((bucket) => {
          const onClick = () => {
            setSelectedBucket(bucket);
          };

          return {
            ...bucket,
            key: bucket.name,
            onClick,
          };
        });
    },
  });

  if (isPending) {
    return <FileTreeSkeleton />;
  }

  if (isError) {
    return (
      <div className="flex h-full w-full flex-col items-center justify-center gap-2 p-8 text-center text-muted-foreground">
        <p className="text-lg font-medium text-destructive">No S3 bucket detected or insufficient permissions.</p>
        <p className="text-sm">{error instanceof Error ? error.message : String(error)}</p>
      </div>
    );
  }

  return (
    <>
      {headerPortalRef.current &&
        createPortal(
          <Button
            variant="ghost"
            size="icon"
            className="text-muted-foreground hover:text-foreground flex h-6 w-6 cursor-pointer items-center justify-center hover:!bg-transparent"
            onClick={async () => {
              await refetch();
            }}
            aria-label="Refresh"
          >
            <RotateCw className={cn("h-4 w-4", isFetching && "animate-spin")} />
          </Button>,
          headerPortalRef.current,
        )}

      <div className="flex h-full overflow-y-hidden" data-testid="bucket-list">
        <FileTree items={data}>
          {(bucket) => {
            const icon = <Folder className="size-5 text-yellow-600" />;

            return (
              <>
                <span className="flex grow items-center gap-2 truncate py-3">
                  {icon} {bucket.name}
                </span>

                <div className="min-w-40 truncate">{bucket.region}</div>

                <div className="flex items-center justify-center truncate">
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <div
                        className="flex h-6 w-12 cursor-pointer items-center justify-center"
                        tabIndex={-1}
                      >
                        <EllipsisVertical aria-label="Open Bucket Actions Menu" />
                      </div>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                      <DropdownMenuItem
                        onClick={(e) => {
                          e.stopPropagation();

                          copyToClipboard(bucket.endpoint_url);

                          toast.success("Bucket URL copied successfully.");
                        }}
                      >
                        Copy Bucket Url
                        <DropdownMenuShortcut>c</DropdownMenuShortcut>
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              </>
            );
          }}
        </FileTree>
      </div>
    </>
  );
}
