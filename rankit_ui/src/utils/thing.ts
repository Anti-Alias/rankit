import { ScrollItem } from "../components/widgets/InfiniteScroll";
import { Page } from "./page";

export interface Thing extends ScrollItem { }

export async function fetchThings(search: string, cursor?: string): Promise<Page<Thing>> {
  await sleep(1000);
  const data = allThings().filter(thing => thing.name.includes(search));
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

function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const pageSize: number = 32;

function allThings(): Thing[] {
  const result: Thing[] = [];
  for (let i = 1; i <= 100; i += 3) {
    result.push({
      id: i,
      name: `Computer Mice ${i}`,
      image: 'images/things/mice.jpg',
    });
    result.push({
      id: i + 1,
      name: `Rice ${i + 1}`,
      image: 'images/things/rice.jpg',
    });
    result.push({
      id: i + 2,
      name: `Apples ${i + 2}`,
      image: 'images/things/apples.jpg',
    });
  }
  return result;
}

