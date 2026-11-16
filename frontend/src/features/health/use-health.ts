import { useQuery } from "@tanstack/react-query"

import { api } from "@/shared/api/client"

/** État du serveur et de sa base, revérifié toutes les 30 secondes. */
export function useHealth() {
  return useQuery({
    queryKey: ["health", "ready"],
    queryFn: async () => {
      const { data, error } = await api.GET("/api/health/ready")
      if (error) throw new Error(error.detail)
      return data
    },
    refetchInterval: 30_000,
  })
}
