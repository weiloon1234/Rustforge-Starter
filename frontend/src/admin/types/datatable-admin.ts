import type { AdminType } from "./enums";
import type {
  DataTableQueryRequestBase,
  DataTableEmailExportRequestBase,
} from "../../shared/types/datatable";

export interface AdminDatatableQueryInput {
  base?: DataTableQueryRequestBase;
  q?: string | null;
  username?: string | null;
  email?: string | null;
  admin_type?: AdminType | null;
}

export interface AdminDatatableEmailExportInput {
  base: DataTableEmailExportRequestBase;
  q?: string | null;
  username?: string | null;
  email?: string | null;
  admin_type?: AdminType | null;
}
