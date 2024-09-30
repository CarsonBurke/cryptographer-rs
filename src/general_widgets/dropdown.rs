// use iced::{
//     widget::{button, column, container, row, Button, Column, Container, Row, Text},
//     Color, Element,
// };
// use iced_aw::DropDown;

// use crate::{constants::spacings, styles};

// pub fn dropdown<'a, Message: 'a + std::clone::Clone>(
//     condition: bool,
//     base_background: Color,
//     button: Element<'a, Message>,
//     dropdown_children_generator: impl Fn() -> Vec<(Row<'a, Message>, Vec<Element<'a, Message>>)>,
// ) -> Container<'a, Message> {
//     container(DropDown::new(
//         button,
//         container(match condition {
//             true => {
//                 Column::with_children({
//                     let mut children = Vec::new();

//                     let dropdown_children = dropdown_children_generator();

//                     for column in dropdown_children {
//                         let column = Column::with_children({
//                             let mut column_children: Vec<Element<'a, Message>> = Vec::new();
//                             let mut index = 0;

//                             for child in column.1 {
//                                 column_children.push(child)
//                             }

//                             column_children
//                         });

//                         children.push(column.into());
//                     }

//                     children
//                 })
//             }
//             false => column![],
//         })
//         .padding(spacings::XSMALL)
//         /* .style(styles::container::test_background_4()), */
//         condition
//     ))
// }
