// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { CancelPreview, ReleaseResource, SequencePreviewRequest, SourcePreviewRequest } from "./api.ts";

export class PreviewClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  preview_source_frame(request: SourcePreviewRequest): Promise<api.SourcePreviewResult> {
    return this.c.call("preview.source_frame", { request }) as Promise<api.SourcePreviewResult>;
  }
  preview_sequence_frame(request: SequencePreviewRequest): Promise<api.SequencePreviewResult> {
    return this.c.call("preview.sequence_frame", { request }) as Promise<api.SequencePreviewResult>;
  }
  preview_cancel(request: CancelPreview): Promise<api.ResourceActionAck> {
    return this.c.call("preview.cancel", { request }) as Promise<api.ResourceActionAck>;
  }
  preview_release(request: ReleaseResource): Promise<api.ResourceActionAck> {
    return this.c.call("preview.release", { request }) as Promise<api.ResourceActionAck>;
  }
}
