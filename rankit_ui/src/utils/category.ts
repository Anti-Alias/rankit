import { ScrollItem } from "../components/widgets/InfiniteScroll";
import { allCategories } from "./db";
import { Page } from "./page";

export interface Category extends ScrollItem { }

const sleepTime: number = 1000;
const pageSize: number = 32;

export async function fetchCategories(search: string, cursor?: string): Promise<Page<Category>> {
  await new Promise(resolve => setTimeout(resolve, sleepTime));
  const data = allCategories().filter(cat => cat.name.includes(search));
  const curs = cursor ? parseInt(cursor) : 0;
  const nextCursor = curs + pageSize;
  if (data.length - 1 > nextCursor) {
    const splicedData = data.splice(curs, nextCursor - curs);
    return { data: splicedData, cursor: nextCursor.toString() };
  }
  else {
    const splicedData = data.splice(curs, nextCursor - curs);
    return { data: splicedData }
  }
}
