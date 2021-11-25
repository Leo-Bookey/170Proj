extern crate ndarray;
extern crate test;
extern crate argmin;
use crate::task::{Task, MaxTask};
use crate::opto::prox::{Prox, UnconstrainedProxCache};
use crate::opto::structs::{validate, HasVariables, ProblemData, Settings, Stats, Variables};
use crate::opto::term::{Objective, Residual, TermCache};
use crate::quadratics::envelope::envelope;
use crate::quadratics::pwq::PiecewiseQuadratic;
use crate::smartcore::linalg;
use crate::benchfun;
use crate::benchmark_functions;
use chrono::Utc;
use ndarray::{stack, Array1, Axis};
use sprs::{bmat, CsMat};
use sprs_suitesparse_ldl::LdlNumeric;
use std::f64;
use smartcore::linalg::naive::dense_matrix::*;
use smartcore::linalg::qr::*;
use numpy::{ToPyArray, PyArray};
use multi::*;
use single::*;
use ndarray::Array1;
use argmin::prelude::*;
use argmin::{ArgminProblem, BacktrackingLineSearch, GDGammaUpdate, GradientDescent};
use argmin::testfunctions::{rosenbrock_derivative_nd, rosenbrock_nd};

// WANT: the best optimization function for expectation of the maximum
// A generalized optimization toolkit -- apply at your own risk 

fn qr_factorize(a: &CsMat<f64>, p: &CsMat<f64>, r: &Array1<f64>, s: &Array1<f64>) -> DenseMatrix {
    let qr = a.qr().unwrap();
    let orthogonal: DenseMatrix<f64> = qr.Q();
    let triangular: DenseMatrix<f64> = qr.R();
    (orthogonal, trianglular)
}

// Assign 1
Struct benchfun::single::Ackley
// Assign 2
Struct benchfun::single::Rastrigin
// Assign 3
Struct benchmark_functions::Rosenbrock
// Assign 4
Struct benchmark_functions::Griewank
// Assign 5
Struct benchmark_functions::Ridge
// Assign 6
Struct benchmark_functions::Sphere
// Assign 7
Struct benchmark_functions::Zakharov

pub fn bencher() -> u32 {
    range(0, 1000).fold(0, |old, new| old ^ new)
    #[bench]
    fn function(b: &mut test::Bencher) {
        b.iter(run);
    }
    #[bench]
    fn closure(b: &mut test::Bencher) {
        b.iter(|| run());
    }
    #[bench]
    fn tester(b: &mut test::Bencher) {
        fn one() {test::Ackley(run()); }
        fn two() {test::Rastrigin(run()); }
        fn three() {test::Rosenbrock(run()); }
        fn four() {test::Griewank(run()); }
        fn five() {test::Ridge(run()); }
        fn six() {test::Sphere(run()); }
        fn seven() {test::Zakharov(run()); }
        b.iter(one);
        b.iter(two);
        b.iter(three);
        b.iter(four);
        b.iter(five);
        b.iter(six);
        b.iter(seven);
    }
}

fn gradient_des() {
    let cost = |x: &Array1<f64>| -> f64 { rosenbrock_nd(x, 1_f64, 100_f64) };
    let gradient = |x: &Array1<f64>| -> Array1<f64> { rosenbrock_derivative_nd(x, 1_f64, 100_f64) };
    let mut prob = ArgminProblem::new(&cost);
    prob.gradient(&gradient);
    let mut solver = GradientDescent::new();

    solver.max_iters(10_000);
    solver.gamma_update(GDGammaUpdate::BarzilaiBorwein);

    let init_param: Array1<f64> = Array1::from_vec(vec![1.5, 1.5]);
    println!("{:?}", init_param);

    let result = solver.run(&prob, &init_param).unwrap();

    println!("{:?}", result);

    let mut solver = GradientDescent::new();
    solver.init(&prob, &init_param).unwrap();
    solver.max_iters(10_000);
    solver.gamma_update(GDGammaUpdate::BarzilaiBorwein);

    let mut cter;
    loop {
        cter = solver.next_iter().unwrap();
        if cter.terminated {
            break;
        };
    }
}

fn factorize_kkt(v1: &CsMat<f64>, v2: &CsMat<f64>, v3: &Array1<f64>, v4: &Array1<f64>) -> LdlNumeric {
    let v4_diag = CsMat::new_csc(
        (v4.len(), v4.len()),
        (0..=v4.len()).collect(),
        (0..v4.len()).collect(),
        v4.to_vec(),
    );
    let tl = v2 + &v4_diag;
    let vinv = -1. / v3;
    let br = CsMat::new_csc(
        (v3.len(), v3.len()),
        (0..=v3.len()).collect(),
        (0..v3.len()).collect(),
        vinv.to_vec(),
    );
    let k = bmat(&[
        [Some(tl.view()), Some(v1.transpose_view())],
        [Some(v1.view()), Some(br.view())],
    ]);

    match LdlNumeric::new(k.view()) {
        Ok(v) => v
    }
}

fn solve_kkt(
    variables: &Variables,
    problem_data: &ProblemData,
    rho: &Array1<f64>,
    sigma: &Array1<f64>,
    ldl: &LdlNumeric,
) -> (Array1<f64>, Array1<f64>) {
    let top = sigma * &variables.x - &variables.w - &problem_data.q;
    let bottom = &variables.z - &(&variables.y / rho);
    let right_hand_side = stack![Axis(0), top, bottom];
    let split_index = variables.n_vars();
    let mut solution = ldl.solve(&right_hand_side.to_vec());
    let nu = Array1::from(solution.split_off(split_index));
    let next_xt = Array1::from(solution);
    let next_zt = &variables.z + &(&(&nu - &variables.y) / rho);
    (next_xt, next_zt)
}

fn step<U: Prox + ?Sized>(
    variables: &mut Variables,
    problem_data: &ProblemData,
    settings: &Settings,
    ldl: &LdlNumeric,
    pc: &mut U,
) {
    let (next_xt, next_zt) = solve_kkt(
        variables,
        problem_data,
        &settings.rho,
        &settings.sigma,
        &ldl,
    );
    let prox_arg = settings.alpha * &next_xt
        + (1. - settings.alpha) * &variables.x
        + &(&variables.w / &settings.sigma);
    let next_x = pc.prox(problem_data, settings, &prox_arg);
    let next_w = &variables.w
        + &(&settings.sigma
            * &(settings.alpha * &next_xt + (1. - settings.alpha) * &variables.x - next_x));
    let next_y = &variables.y + &(settings.alpha * &settings.rho * &(&next_zt - &problem_data.b));

    variables.xt = next_xt;
    variables.zt = next_zt;
    variables.x = next_x.to_owned();
    variables.w = next_w;
    variables.y = next_y;
}

fn _optimize<T: Objective + Residual>(
    ldl: &LdlNumeric,
    pc: &mut UnconstrainedProxCache,
    tc: &mut TermCache,
    variables: &mut Variables,
    problem_data: &ProblemData,
    settings: &Settings,
    stats: &mut Stats,
    obj_res_calc: &T,
) -> Variables {
    let mut iters = 0;
    while iters < settings.max_iter
        && !tc.terminate(obj_res_calc, variables, problem_data, settings, iters)
    {
        iters += 1;
        step(variables, problem_data, settings, ldl, pc);
        stats.update(
            variables,
            problem_data,
            obj_res_calc,
            settings.compute_stats,
        );
    }
    stats.iters += iters;
    stats.prox_iters = pc.call_count();
    variables.to_owned()
}

pub fn optimize_structs<T: Objective + Residual>(
    settings: &Settings,
    problem_data: &mut ProblemData,
    variables: &mut Variables,
    obj_res_calc: &T,
    convexify: bool,
) -> (Variables, Stats) {
    let begin_solve = Utc::now();
    validate(variables, settings, problem_data);
    variables.z = problem_data.b.to_owned();
    let mut term_cache = TermCache::new(problem_data, settings);
    let mut stats = Stats::new(settings.max_iter);
    let ldl = factorize_kkt(
        &problem_data.a,
        &problem_data.p,
        &settings.rho,
        &settings.sigma,
    );

    if convexify {
        let g_orig = problem_data.g.to_owned();
        for i in 0..variables.n_vars() {
            problem_data.g[i] = envelope(&g_orig[i]);
        }
        let mut prox_cache = UnconstrainedProxCache::new(&problem_data, &settings);
        let sol = _optimize(
            &ldl,
            &mut prox_cache,
            &mut term_cache,
            variables,
            problem_data,
            settings,
            &mut stats,
            obj_res_calc,
        );
        variables.x = sol.x;
        variables.z = sol.z;
        variables.w = sol.w;
        variables.y = sol.y;
        variables.xt = sol.xt;
        variables.zt = sol.zt;
        problem_data.g = g_orig;
    }

    term_cache.clear();
    let mut prox_cache = UnconstrainedProxCache::new(&problem_data, settings);
    let sol = _optimize(
        &ldl,
        &mut prox_cache,
        &mut term_cache,
        variables,
        problem_data,
        settings,
        &mut stats,
        obj_res_calc,
    );
    stats.solve_time_ms = (Utc::now() - begin_solve).num_milliseconds() as usize;
    stats
        .objective
        .push(obj_res_calc.objective(&sol, &problem_data));
    stats
        .residual
        .push(obj_res_calc.residual(&sol, &problem_data));
    (sol, stats)
}
#[allow(non_snake_case)]
pub fn optimize<T: Objective + Residual>(
    A: CsMat<f64>,
    b: Array1<f64>,
    P: CsMat<f64>,
    q: Array1<f64>,
    g: Array1<PiecewiseQuadratic>,
    o: &T,
    convexify: bool,
) -> (Variables, Stats) {
    let (m, n) = A.shape();
    let settings = Settings::defaults(n, m);
    let mut problem_data = ProblemData::new(P, q, A, b, g);
    let mut variables = Variables::from_problem_data(&problem_data);
    optimize_structs(&settings, &mut problem_data, &mut variables, o, convexify)
}

