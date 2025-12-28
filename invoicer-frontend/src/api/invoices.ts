import type { Invoice } from "../types/invoice.ts";

const API_URL = "http://localhost:3000";

export async function fetchInvoices(params?: {
  sort?: string;
  clientId?: number;
}): Promise<Invoice[]> {
  const query = new URLSearchParams();

  if (params?.sort) query.set("sort", params.sort);
  if (params?.clientId) query.set("client_id", params.clientId.toString());

  const res = await fetch(`${API_URL}/invoices?${query.toString()}`);

  if (!res.ok) {
    throw new Error("Failed to fetch invoices");
  }

  return res.json();
}
