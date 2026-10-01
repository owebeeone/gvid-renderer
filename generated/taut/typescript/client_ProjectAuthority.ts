// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { AcquireWriter } from "./api.ts";

export class ProjectAuthorityClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  authority_open(project_id: string): Promise<api.ProjectOpen> {
    return this.c.call("authority.open", { project_id }) as Promise<api.ProjectOpen>;
  }
  authority_acquire_writer(request: AcquireWriter): Promise<api.WriterLease> {
    return this.c.call("authority.acquire_writer", { request }) as Promise<api.WriterLease>;
  }
}
