// GENERATED server stubs: a handler interface + IR-driven registration.
import type * as api from "./api.ts";
import type { AcquireWriter } from "./api.ts";

export interface ProjectAuthorityHandlers {
  authority_open(project_id: string): Promise<api.ProjectOpen>;
  authority_acquire_writer(request: AcquireWriter): Promise<api.WriterLease>;
}

// Register against the IR (the transport reads kind/params from the contract):
export function register(transport: any, schema: any, h: ProjectAuthorityHandlers): void {
  const bind: Record<string, unknown> = {
    "authority.open": h.authority_open.bind(h),
    "authority.acquire_writer": h.authority_acquire_writer.bind(h),
  };
  for (const m of schema.services["ProjectAuthority"].methods) transport.registerMethod(m, bind[m.name]);
}
