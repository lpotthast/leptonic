use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    move_axis::AxisExample, move_basic::BasicMovementExample,
    move_constrain_center::ConstrainCenterExample, move_constrained::ConstrainedBasicExample,
    move_container_click::ContainerClickExample, move_programmatic::ProgrammaticExample,
};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageUseMove() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-move" class="anchor">
                "use_move"
                <AnchorLink href="#use-move" description="Direct link to section: use_move"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_move"</Code>" hook tracks pointer and keyboard movement. "
                "Supports arrow key navigation, text selection management, and optional area-constrained movement via "<Code inline=true>"MoveConstraint"</Code>". "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useMove.html" target=LinkTarget::_Blank>
                    "useMove"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UseMoveInput"</Code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Disables movement when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"axis"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<MoveAxis>>"</Code></TableCell>
                            <TableCell>"Optional axis constraint: Horizontal, Vertical, or Both (default when None)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_move_start"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<MoveStartEvent>>"</Code></TableCell>
                            <TableCell>"Called when movement starts. Provides pointer_type, modifiers, page_x, page_y."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_move"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<MoveEvent>>"</Code></TableCell>
                            <TableCell>"Called during movement. Provides delta_x, delta_y, pointer_type, modifiers."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_move_end"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<MoveEndEvent>>"</Code></TableCell>
                            <TableCell>"Called when movement ends."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"constraint"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<MoveConstraint>"</Code></TableCell>
                            <TableCell>"Optional area-bounded movement configuration (is_rtl, constrain_center, allow_container_click, initial_position)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseMoveReturn"</Code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseMoveProps"</Code></TableCell>
                            <TableCell>"Spread onto the movable element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_moving"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the element is currently being moved."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"constraint"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<UseMoveConstraintReturn>"</Code></TableCell>
                            <TableCell>"Present when constraint was configured. Provides container_props, normalized_position, pixel_position, set_position."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="basic-movement" class="anchor">
                "Basic Movement"
                <AnchorLink href="#basic-movement" description="Direct link to section"/>
            </h2>

            <p>"Unconstrained drag and keyboard movement with event logging."</p>

            <DemoShell source=include_str!("demos/move_basic.rs")>
                <BasicMovementExample />
            </DemoShell>

            <h2 id="constrained" class="anchor">
                "Constrained Movement"
                <AnchorLink href="#constrained" description="Direct link to section"/>
            </h2>

            <p>"Constrain element movement within a container boundary using a " <code>"MoveConstraint"</code> " configuration."</p>

            <DemoShell source=include_str!("demos/move_constrained.rs")>
                <ConstrainedBasicExample />
            </DemoShell>

            <h2 id="axis-constraint" class="anchor">
                "Axis Constraint"
                <AnchorLink href="#axis-constraint" description="Direct link to section"/>
            </h2>

            <p>"Constrain movement to horizontal or vertical axis only."</p>

            <DemoShell source=include_str!("demos/move_axis.rs")>
                <AxisExample />
            </DemoShell>

            <h2 id="container-click" class="anchor">
                "Container Click"
                <AnchorLink href="#container-click" description="Direct link to section"/>
            </h2>

            <p>"When enabled, clicking the container moves the element to that position."</p>

            <DemoShell source=include_str!("demos/move_container_click.rs")>
                <ContainerClickExample />
            </DemoShell>

            <h2 id="constrain-center" class="anchor">
                "Constrain Center"
                <AnchorLink href="#constrain-center" description="Direct link to section"/>
            </h2>

            <p>"Compare constraining element bounds vs element center."</p>

            <DemoShell source=include_str!("demos/move_constrain_center.rs")>
                <ConstrainCenterExample />
            </DemoShell>

            <h2 id="programmatic" class="anchor">
                "Programmatic Control"
                <AnchorLink href="#programmatic" description="Direct link to section"/>
            </h2>

            <p>"Use set_position to programmatically move the element."</p>

            <DemoShell source=include_str!("demos/move_programmatic.rs")>
                <ProgrammaticExample />
            </DemoShell>

            <h2 id="deviations" class="anchor">
                "Deviations"
                <AnchorLink href="#deviations" description="Direct link to section: Deviations"/>
            </h2>

            <ul>
                <li><Code inline=true>"axis"</Code>" constraint as "<Code inline=true>"Signal<Option<MoveAxis>>"</Code>" (not in react-aria)"</li>
                <li><Code inline=true>"page_x"</Code>"/"<Code inline=true>"page_y"</Code>" on MoveStartEvent (not in react-aria)"</li>
                <li><Code inline=true>"MoveConstraint"</Code>" system for area-bounded movement (leptonic-specific)"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=crate::routes::doc::interactions::Dnd.materialize()>"Drag & Drop"</Link></li>
                <li><Link href=crate::routes::doc::slider::Hook.materialize()>"use_slider"</Link>" (uses use_move internally)"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_move", link: "#use-move" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Basic Movement", link: "#basic-movement" },
                Toc::Leaf { title: "Constrained Movement", link: "#constrained" },
                Toc::Leaf { title: "Axis Constraint", link: "#axis-constraint" },
                Toc::Leaf { title: "Container Click", link: "#container-click" },
                Toc::Leaf { title: "Constrain Center", link: "#constrain-center" },
                Toc::Leaf { title: "Programmatic Control", link: "#programmatic" },
                Toc::Leaf { title: "Deviations", link: "#deviations" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
