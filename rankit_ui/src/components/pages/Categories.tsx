import { fetchCategories } from "../../utils/category";
import { InfiniteScroll } from "../widgets/InfiniteScroll";

export function Categories() {
  return <InfiniteScroll
    searchPlaceholder="Search Categories"
    notFoundText="Categories not found"
    fetcher={fetchCategories}
  />;
}

