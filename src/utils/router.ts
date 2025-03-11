import { useRouter } from "vue-router";

export function goTo(path: string) {
  const router = useRouter();
  router.push(path);
}