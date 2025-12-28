import { useEffect, useState } from "react";
import { fetchInvoices } from "../api/invoices.ts";
import type { Invoice } from "../types/invoice.ts";

export default function InvoiceList() {
  const [invoices, setInvoices] = useState<Invoice[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchInvoices({ sort: "issued_at" })
      .then(setInvoices)
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <p>Loading...</p>;

  return (
    <section>
      <ul>
        {invoices.map((invoice) => (
          <li key={invoice.id}>
            Invoice #{invoice.id} {invoice.client_name} - {invoice.status}
          </li>
        ))}
      </ul>
    </section>
  );
}
