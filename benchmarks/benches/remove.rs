use aatree::AATreeSet;
use criterion::{
	criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
	BenchmarkId, Criterion
};
use std::{collections::BTreeSet, rc::Rc, sync::Mutex, time::Duration};

macro_rules! benchmark {
	($ty:ty, $amount:expr, hit) => {
		benchmark!(@internal, $ty, $amount, (0..$amount).map(|x| x*2), (0..$amount).map(|x| x*2), hit);
	};
	($ty:ty, $amount:expr, miss) => {
		benchmark!(@internal, $ty, $amount, (0..$amount).map(|x| x*2), (0..$amount).map(|x| x*2+1), miss);
	};
	(@internal, $ty:ty, $amount:expr, $iter_fill:expr, $iter_test:expr, $success:ident) => {
		paste::item! {
			fn [<$ty:lower _remove_ $amount _ $success>](container: &Rc<Mutex<$ty<u64>>>, test: &[u64]) {
				let mut container = container.lock().unwrap();
				for i in test {
					container.remove(i);
				}
			}
			fn [<bench_ $ty:lower _remove_ $amount _ $success>]<M: Measurement>(g: &mut BenchmarkGroup<M>, id: BenchmarkId) {
				let container: $ty<u64> = $iter_fill.collect();
				let test: Vec<u64> = $iter_test.collect();
				let container = Rc::new(Mutex::new(container));
				g.bench_with_input(id, &(container, test), |b, (c, t)| b.iter(|| [<$ty:lower _remove_ $amount _ $success>](c, t)));
			}
		}
	};
	($group:literal = [$(($name:literal: $ty:ty, $amount:expr, $success:ident)),+]) => {
		$(benchmark!($ty, $amount, $success);)+
		paste::item! {
			fn [<bench_ $group:lower>](c: &mut Criterion) {
				let mut g = c.benchmark_group($group);
				g.sample_size(150).measurement_time(Duration::from_secs(20));
				$([<bench_ $ty:lower _remove_ $amount _ $success>](&mut g, BenchmarkId::new(format!("{}_{}", $name, stringify!($success)), $amount));)+
				g.finish();
			}
		}
	};
}

benchmark!(
	"Remove" = [
		("AATree": AATreeSet, 10000, hit),
		("AATree": AATreeSet, 10000, miss),
		("AATree": AATreeSet, 100000, hit),
		("AATree": AATreeSet, 100000, miss),
		("BTree": BTreeSet, 10000, hit),
		("BTree": BTreeSet, 10000, miss),
		("BTree": BTreeSet, 100000, hit),
		("BTree": BTreeSet, 100000, miss)
	]
);

criterion_group!(benches, bench_remove);
criterion_main!(benches);
