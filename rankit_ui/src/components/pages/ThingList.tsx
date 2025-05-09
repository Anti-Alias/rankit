import { fetchThings } from "../../utils/thing";
import { InfiniteScroll } from "../widgets/InfiniteScroll";

export function ThingList() {
  return <InfiniteScroll
    searchPlaceholder="Search Things"
    notFoundText="Things not found"
    fetcher={fetchThings}
  />;
}

