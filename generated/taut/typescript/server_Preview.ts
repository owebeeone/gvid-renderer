// GENERATED server stubs: a handler interface + IR-driven registration.
import type * as api from "./api.ts";
import type { CancelPreview, ReleaseResource, SequencePreviewRequest, SourcePreviewRequest } from "./api.ts";

export interface PreviewHandlers {
  preview_source_frame(request: SourcePreviewRequest): Promise<api.SourcePreviewResult>;
  preview_sequence_frame(request: SequencePreviewRequest): Promise<api.SequencePreviewResult>;
  preview_cancel(request: CancelPreview): Promise<api.ResourceActionAck>;
  preview_release(request: ReleaseResource): Promise<api.ResourceActionAck>;
}

// Register against the IR (the transport reads kind/params from the contract):
export function register(transport: any, schema: any, h: PreviewHandlers): void {
  const bind: Record<string, unknown> = {
    "preview.source_frame": h.preview_source_frame.bind(h),
    "preview.sequence_frame": h.preview_sequence_frame.bind(h),
    "preview.cancel": h.preview_cancel.bind(h),
    "preview.release": h.preview_release.bind(h),
  };
  for (const m of schema.services["Preview"].methods) transport.registerMethod(m, bind[m.name]);
}
