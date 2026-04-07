<script lang="ts">
  import type { AccountStatus, KycStatus, AmlStatus, TransactionStatus } from '@/lib/types';

  type Status = AccountStatus | KycStatus | AmlStatus | TransactionStatus | 'compliant' | 'warning' | 'breach';

  interface Props {
    status: Status;
    label?: string;
  }

  const { status, label }: Props = $props();

  const statusConfig: Record<Status, { bg: string; text: string; dot: string; defaultLabel: string }> = {
    active: { bg: 'bg-green-50', text: 'text-green-700', dot: 'bg-green-500', defaultLabel: 'Actif' },
    suspended: { bg: 'bg-amber-50', text: 'text-amber-700', dot: 'bg-amber-500', defaultLabel: 'Suspendu' },
    closed: { bg: 'bg-red-50', text: 'text-red-700', dot: 'bg-red-500', defaultLabel: 'Fermé' },
    pending: { bg: 'bg-gray-50', text: 'text-gray-700', dot: 'bg-gray-500', defaultLabel: 'En attente' },
    submitted: { bg: 'bg-blue-50', text: 'text-blue-700', dot: 'bg-blue-500', defaultLabel: 'Soumis' },
    approved: { bg: 'bg-green-50', text: 'text-green-700', dot: 'bg-green-500', defaultLabel: 'Approuvé' },
    rejected: { bg: 'bg-red-50', text: 'text-red-700', dot: 'bg-red-500', defaultLabel: 'Rejeté' },
    clear: { bg: 'bg-green-50', text: 'text-green-700', dot: 'bg-green-500', defaultLabel: 'Clair' },
    warning: { bg: 'bg-amber-50', text: 'text-amber-700', dot: 'bg-amber-500', defaultLabel: 'Attention' },
    alert: { bg: 'bg-orange-50', text: 'text-orange-700', dot: 'bg-orange-500', defaultLabel: 'Alerte' },
    blocked: { bg: 'bg-red-50', text: 'text-red-700', dot: 'bg-red-500', defaultLabel: 'Bloqué' },
    completed: { bg: 'bg-green-50', text: 'text-green-700', dot: 'bg-green-500', defaultLabel: 'Complété' },
    failed: { bg: 'bg-red-50', text: 'text-red-700', dot: 'bg-red-500', defaultLabel: 'Échoué' },
    reversed: { bg: 'bg-purple-50', text: 'text-purple-700', dot: 'bg-purple-500', defaultLabel: 'Inversé' },
    compliant: { bg: 'bg-green-50', text: 'text-green-700', dot: 'bg-green-500', defaultLabel: 'Conforme' },
    breach: { bg: 'bg-red-50', text: 'text-red-700', dot: 'bg-red-500', defaultLabel: 'Dépassement' },
  };

  const config = statusConfig[status];
  const displayLabel = label || config.defaultLabel;
</script>

<span
  class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium {config.bg} border {config.text}"
  aria-label={displayLabel}
>
  <span class="h-2 w-2 rounded-full {config.dot}" aria-hidden="true"></span>
  {displayLabel}
</span>
