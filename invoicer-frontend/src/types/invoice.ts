export type InvoiceStatus = "draft" | "sent" | "paid" | "overdue";

export interface Invoice {
  id: number;
  client_id: number;
  client_name: string;
  status: InvoiceStatus;
  issued_at: string;
  due_at: string | null;
}
