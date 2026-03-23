use crate::{game::Repository, models::GameResult}; 
use yew_icons::{Icon, IconData};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RecordRowProps {
    pub record: GameResult,
}

#[function_component(RecordRow)]
pub fn record_row(props: &RecordRowProps) -> Html {
    let r = &props.record;

    let duration_secs = r.duration.num_seconds();

    let row_class = if r.has_won {
        "border-b hover:bg-white/5 odd:bg-white/5 even:bg-transparent"
    } else {
        "border-b hover:bg-white/5 odd:bg-white/5 even:bg-transparent"
    };

    let has_won = if r.has_won {
        html! {
            <span class="flex items-center gap-1 text-green-400">
                <Icon data={IconData::LUCIDE_CHECK} width={"18px"} />
                {"Win"}
            </span>
        }
    } else {
        html! {
            <span class="flex items-center gap-1 text-red-400">
                <Icon data={IconData::LUCIDE_X} width={"18px"} />
                {"Loss"}
            </span>
        }
    };

    html! {
        <tr class={row_class}>
            <td class="p-2">
                {r.started_at.format("%Y-%m-%d %H:%M:%S").to_string()}
            </td>

            <td class="p-2">
                {format!("{}s", duration_secs)}
            </td>

            <td class="p-2">
                {has_won}
            </td>
            

            <td class="p-2">
                {format!("{} × {}", r.rows, r.columns)}
            </td>

            <td class="p-2">{r.mines_count}</td>
            <td class="p-2">{r.revealed_count}</td>
            <td class="p-2">{r.flags_count}</td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct RecordsTableProps {
    pub records: Vec<GameResult>,
}

#[function_component(RecordsTable)]
pub fn records_table(props: &RecordsTableProps) -> Html {
    html! {
        <>
            <div class="text-center p-1">
                <h2 class="text-2xl font-semibold text-white">{"Recent Games"}</h2>
            </div>

            <table class="table-auto w-full text-left border-collapse text-white bg-black/50">
                <thead>
                    <tr class="border-b">
                        <th class="p-2">{"Started At"}</th>
                        <th class="p-2">{"Duration"}</th>
                        <th class="p-2">{"Result"}</th>
                        <th class="p-2">{"Grid"}</th>
                        <th class="p-2">{"Mines"}</th>
                        <th class="p-2">{"Revealed"}</th>
                        <th class="p-2">{"Flags"}</th>
                    </tr>
                </thead>

                <tbody>
                    {props.records.iter().map(|r| {
                        html! {
                            <RecordRow record={r.clone()} />
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        </>
    }
}


#[function_component(Records)]
pub fn records() -> Html {
    let repository = unsafe { use_context::<Repository>().unwrap_unchecked() };
    let records = repository.get_last_records();

    if records.is_empty() {
        return html! { <p>{"No records yet"}</p> };
    }

    html! {
        <div class="flex flex-col w-full">
            <RecordsTable records={records.to_vec()} />
        </div>
    }
}