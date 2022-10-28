import { writable } from "svelte/store";

export const pageCount: any = writable(0);
export const pageLimit: any = writable(10);
export const pageNumber: any = writable(1);

export async function setPagination(limit: number, page: number) {
  pageLimit.update((l: number) => limit);
  pageNumber.update((p: number) => page);
}
