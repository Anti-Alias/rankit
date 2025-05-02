import { fetchThings } from "../../utils/thing";
import { InfiniteScroll } from "../widgets/InfiniteScroll";

export function Things() {
  return <InfiniteScroll
    searchPlaceholder="Search Things"
    notFoundText="Things not found"
    fetcher={fetchThings}
  />;
}

