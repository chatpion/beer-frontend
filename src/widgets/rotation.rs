use orbtk::prelude::*;

use crate::{
    widgets::angle::{AngleView, AngleType}
};

#[derive(Default, AsAny)]
pub struct RotationViewState {}

impl State for RotationViewState {

}


widget!(RotationView<RotationViewState> {});

impl Template for RotationView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("RotationView").child(
            Stack::new().spacing(10.0).child(
                TextBlock::new()
                    .text("Tourner le téléscope")
                    .build(ctx)
            ).child(
                Stack::new().orientation("horizontal")
                    .child(
                        TextBlock::new()
                            .text("Ascension droite : ")
                            .build(ctx)
                    )
                    .child(
                        AngleView::new()
                            .angle_type(AngleType::RightAsc)
                            .first_angle(false)
                            .build(ctx)
                    )
                    .build(ctx)
            ).child(
                Stack::new().orientation("horizontal")
                    .child(
                        TextBlock::new()
                            .text("Déclinaison")
                            .build(ctx)
                    ).child(
                        AngleView::new()
                            .angle_type(AngleType::Declination)
                            .first_angle(true)
                            .build(ctx)
                    ).build(ctx)
            ).build(ctx)
        )
    }
}
