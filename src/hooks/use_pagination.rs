use std::fmt::Debug;

use leptos::prelude::{signal, Get, Memo, ReadSignal, Signal, WriteSignal};

pub struct PaginationData<T: Send + Sync + PartialEq + Clone + Debug + 'static> {
    pub current_page: ReadSignal<usize>,
    pub set_current_page: WriteSignal<usize>,
    pub data: Memo<Vec<T>>,
    pub total_pages: Memo<usize>,
}

pub fn use_pagination<N, T, M>(data: N, limit: M) -> PaginationData<T>
where
    T: Send + Sync + PartialEq + Clone + Debug + 'static,
    N: Fn() -> Vec<T> + Send + Sync + 'static,
    M: Fn() -> usize + Send + Sync + Clone + Copy + 'static,
{
    // TODO - Remove memo spam. This should not spam memo as it does here

    let (current_page, set_current_page) = signal(1);

    let memoised_data = Memo::new(move |_| data());

    let paginated_data = Memo::new(move |_| {
        let data = memoised_data.get();
        let start = (current_page.get() - 1) * limit();
        let end = (current_page.get() * limit()).min(data.len());

        let val: Vec<T> = data[(start as usize)..(end as usize)].into();

        return val;
    });

    let total_pages =
        Memo::new(move |_| (((memoised_data.get().len() as f32) / limit() as f32).ceil() as usize));

    PaginationData {
        current_page,
        set_current_page,
        data: paginated_data,
        total_pages,
    }
}
