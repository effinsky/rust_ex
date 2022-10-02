fn main() {
	// WIP: how to do a param type alias; how to eliminate reliance on clone trait
	// sorting a list in place
	let mut list = vec![4i64, -200, 1, 6, 34, 11, 53, 2];
	println!("list before sorting: {:?}", &list);
	quicksort(&mut list);
	println!("list after sorting: {:?}", &list);
}

pub fn quicksort<T>(list: &mut [T])
where
	T: PartialOrd + Clone,
{
	quicksort_internal(list, 0_usize, list.len() - 1);
}

fn quicksort_internal<T>(list: &mut [T], low: usize, high: usize)
where
	T: PartialOrd + Clone,
{
	if low < high {
		let pivot_idx = partition(list, low, high);
		quicksort_internal(list, low, pivot_idx - 1);
		quicksort_internal(list, pivot_idx + 1, high);
	}
}

fn partition<T>(list: &mut [T], first: usize, last: usize) -> usize
where
	T: PartialOrd + Clone,
{
	let pivot = list[first].clone();
	let mut low = first + 1;
	let mut high = last;

	while low < high {
		while low <= high && list[low] <= pivot {
			low += 1;
		}
		while low <= high && list[high] > pivot {
			high -= 1;
		}
		if low < high {
			list.swap(low, high);
		}
	}

	while high > low && list[high] >= pivot {
		high -= 1;
	}

	if pivot > list[high] {
		list.swap(first, high);
		return high;
	}

	first
}
