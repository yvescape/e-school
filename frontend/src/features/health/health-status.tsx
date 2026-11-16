import { Badge } from "@/components/ui/badge"
import { Skeleton } from "@/components/ui/skeleton"

import { useHealth } from "./use-health"

export function HealthStatus() {
  const { isPending, isError } = useHealth()

  if (isPending) {
    return (
      <Skeleton
        role="status"
        aria-label="Vérification du serveur"
        className="h-5 w-36"
      />
    )
  }

  return isError ? (
    <Badge variant="destructive">Serveur indisponible</Badge>
  ) : (
    <Badge variant="secondary">Serveur opérationnel</Badge>
  )
}
