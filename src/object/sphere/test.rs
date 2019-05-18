use super::*;

type Subject = Sphere;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_sphere_instance() {
        let expected = Subject {
            center: Vec::new(1.0, 0.0, 0.0),
            radius: 5.7,
            color: Color::new(5.0, 0.0, 5.0),
            reflectance: 0.0,
        };

        let actual = Subject::new(Vec::new(1.0, 0.0, 0.0), 5.7, Color::new(5.0, 0.0, 5.0), 0.0);

        assert_eq!(actual, expected);
    }
}

mod hit {
    use super::*;

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_at_the_center() {
        let sphere = Subject::new(Vec::new(0.0, 0.0, 0.0), 1.0, Color::new(0.0, 0.0, 0.0), 0.0);
        let ray = Ray { origin: Vec::new(-2.0, 0.0, 0.0), direction: Vec::new(1.0, 0.0, 0.0) };

        assert_eq!(sphere.hit(&ray, 0.0, std::f64::INFINITY).unwrap().t, 1.0);
    }

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_from_the_other_direction() {
        let sphere = Subject::new(Vec::new(0.0, 0.0, 0.0), 1.0, Color::new(0.0, 0.0, 0.0), 0.0);
        let ray = Ray { origin: Vec::new(2.0, 0.0, 0.0), direction: Vec::new(-1.0, 0.0, 0.0) };

        assert_eq!(sphere.hit(&ray, 0.0, std::f64::INFINITY).unwrap().t, 1.0);
    }

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_from_the_top() {
        let sphere = Subject::new(Vec::new(0.0, 0.0, 0.0), 1.0, Color::new(0.0, 0.0, 0.0), 0.0);
        let ray = Ray { origin: Vec::new(0.0, 2.0, 0.0), direction: Vec::new(0.0, -1.0, 0.0) };

        assert_eq!(sphere.hit(&ray, 0.0, std::f64::INFINITY).unwrap().t, 1.0);
    }

    #[test]
    fn it_returns_the_distance_for_a_ray_pointing_from_the_diagonal() {
        let sphere = Subject::new(Vec::new(0.0, 0.0, 0.0), 1.0, Color::new(0.0, 0.0, 0.0), 0.0);
        let ray = Ray { origin: Vec::new(2.0, 2.0, 0.0), direction: Vec::new(-1.0, -1.0, 0.0).normalize() };

        if let Some(hit) = sphere.hit(&ray, 0.0, std::f64::INFINITY) {
            assert!((hit.t - 1.828).abs() <= 0.01);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_returns_none_for_a_ray_pointing_away_from_the_sphere() {
        let sphere = Subject::new(Vec::new(0.0, 0.0, 0.0), 1.0, Color::new(0.0, 0.0, 0.0), 0.0);
        let ray = Ray { origin: Vec::new(2.0, 0.0, 0.0), direction: Vec::new(1.0, 0.0, 0.0) };

        assert_eq!(sphere.hit(&ray, 0.0, std::f64::INFINITY), None);
    }
}
