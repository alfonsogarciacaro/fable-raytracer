import { Record } from "fable-library/Types.js";
import { array_type, class_type, record_type, float64_type } from "fable-library/Reflection.js";
import { value as value_1 } from "fable-library/Option.js";
import { max, comparePrimitives, min } from "fable-library/Util.js";
import { op_UnaryNegation_Int32 } from "fable-library/Int32.js";
import { printf, toConsole } from "fable-library/String.js";

export class Vector extends Record {
    constructor(X, Y, Z) {
        super();
        this.X = X;
        this.Y = Y;
        this.Z = Z;
    }
}

export function Vector$reflection() {
    return record_type("RayTracer.Vector", [], Vector, () => [["X", float64_type], ["Y", float64_type], ["Z", float64_type]]);
}

export function Vector_op_Multiply_73181070(k, v) {
    return new Vector(k * v.X, k * v.Y, k * v.Z);
}

export function Vector_op_Subtraction_4C6022A0(v1, v2) {
    return new Vector(v1.X - v2.X, v1.Y - v2.Y, v1.Z - v2.Z);
}

export function Vector_op_Addition_4C6022A0(v1, v2) {
    return new Vector(v1.X + v2.X, v1.Y + v2.Y, v1.Z + v2.Z);
}

export function Vector_Dot_4C6022A0(v1, v2) {
    return ((v1.X * v2.X) + (v1.Y * v2.Y)) + (v1.Z * v2.Z);
}

export function Vector_Mag_5620FEEB(v) {
    return Math.sqrt(((v.X * v.X) + (v.Y * v.Y)) + (v.Z * v.Z));
}

export function Vector_Norm_5620FEEB(v) {
    const mag = Vector_Mag_5620FEEB(v);
    const div = (mag === 0) ? Number.POSITIVE_INFINITY : (1 / mag);
    return Vector_op_Multiply_73181070(div, v);
}

export function Vector_Cross_4C6022A0(v1, v2) {
    return new Vector((v1.Y * v2.Z) - (v1.Z * v2.Y), (v1.Z * v2.X) - (v1.X * v2.Z), (v1.X * v2.Y) - (v1.Y * v2.X));
}

export class Color extends Record {
    constructor(R, G, B) {
        super();
        this.R = R;
        this.G = G;
        this.B = B;
    }
}

export function Color$reflection() {
    return record_type("RayTracer.Color", [], Color, () => [["R", float64_type], ["G", float64_type], ["B", float64_type]]);
}

export function Color_Scale_Z55C8D3EC(k, v) {
    return new Color(k * v.R, k * v.G, k * v.B);
}

export function Color_op_Addition_Z1F82920(v1, v2) {
    return new Color(v1.R + v2.R, v1.G + v2.G, v1.B + v2.B);
}

export function Color_op_Multiply_Z1F82920(v1, v2) {
    return new Color(v1.R * v2.R, v1.G * v2.G, v1.B * v2.B);
}

export function Color_get_White() {
    return new Color(1, 1, 1);
}

export function Color_get_Grey() {
    return new Color(0.5, 0.5, 0.5);
}

export function Color_get_Black() {
    return new Color(0, 0, 0);
}

export function Color_get_Background() {
    return Color_get_Black();
}

export function Color_get_DefaultColor() {
    return Color_get_Black();
}

export class Camera {
    constructor(pos, lookAt) {
        this.pos = pos;
        this.forward = Vector_Norm_5620FEEB(Vector_op_Subtraction_4C6022A0(lookAt, this.pos));
        const down = new Vector(0, -1, 0);
        this.right = Vector_op_Multiply_73181070(1.5, Vector_Norm_5620FEEB(Vector_Cross_4C6022A0(this.forward, down)));
        this.up = Vector_op_Multiply_73181070(1.5, Vector_Norm_5620FEEB(Vector_Cross_4C6022A0(this.forward, this.right)));
    }
}

export function Camera$reflection() {
    return class_type("RayTracer.Camera", void 0, Camera);
}

export function Camera_$ctor_4C6022A0(pos, lookAt) {
    return new Camera(pos, lookAt);
}

export function Camera__get_Pos(c) {
    return c.pos;
}

export function Camera__get_Forward(c) {
    return c.forward;
}

export function Camera__get_Up(c) {
    return c.up;
}

export function Camera__get_Right(c) {
    return c.right;
}

export class Ray extends Record {
    constructor(Start, Dir) {
        super();
        this.Start = Start;
        this.Dir = Dir;
    }
}

export function Ray$reflection() {
    return record_type("RayTracer.Ray", [], Ray, () => [["Start", Vector$reflection()], ["Dir", Vector$reflection()]]);
}

export class Intersection extends Record {
    constructor(Thing, Ray, Dist) {
        super();
        this.Thing = Thing;
        this.Ray = Ray;
        this.Dist = Dist;
    }
}

export function Intersection$reflection() {
    return record_type("RayTracer.Intersection", [], Intersection, () => [["Thing", class_type("RayTracer.SceneObject")], ["Ray", Ray$reflection()], ["Dist", float64_type]]);
}

export class Light extends Record {
    constructor(Pos, Color) {
        super();
        this.Pos = Pos;
        this.Color = Color;
    }
}

export function Light$reflection() {
    return record_type("RayTracer.Light", [], Light, () => [["Pos", Vector$reflection()], ["Color", Color$reflection()]]);
}

export class Scene extends Record {
    constructor(Things, Lights, Camera) {
        super();
        this.Things = Things;
        this.Lights = Lights;
        this.Camera = Camera;
    }
}

export function Scene$reflection() {
    return record_type("RayTracer.Scene", [], Scene, () => [["Things", array_type(class_type("RayTracer.SceneObject"))], ["Lights", array_type(Light$reflection())], ["Camera", Camera$reflection()]]);
}

export const RayTracer_maxDepth = 5;

export function RayTracer_NearestIntersection(ray, scene) {
    let acc = void 0;
    const arr = scene.Things;
    for (let idx = 0; idx <= (arr.length - 1); idx++) {
        const x_1 = arr[idx];
        const dist = x_1.Intersect(ray);
        if ((acc == null) ? true : (dist < value_1(acc).Dist)) {
            acc = (new Intersection(x_1, ray, dist));
        }
    }
    return acc;
}

export function RayTracer_TestRay(ray, scene) {
    const matchValue = RayTracer_NearestIntersection(ray, scene);
    if (matchValue != null) {
        const isect = matchValue;
        if (isect.Dist === Number.POSITIVE_INFINITY) {
            return void 0;
        }
        else {
            return isect.Dist;
        }
    }
    else {
        return void 0;
    }
}

export function RayTracer_TraceRay(ray, scene, depth) {
    const matchValue = RayTracer_NearestIntersection(ray, scene);
    if (matchValue != null) {
        const isect = matchValue;
        if (isect.Dist === Number.POSITIVE_INFINITY) {
            return Color_get_Background();
        }
        else {
            return RayTracer_Shade(isect, scene, depth);
        }
    }
    else {
        return Color_get_Background();
    }
}

export function RayTracer_Shade(isect, scene, depth) {
    const d = isect.Ray.Dir;
    const pos = Vector_op_Addition_4C6022A0(Vector_op_Multiply_73181070(isect.Dist, d), isect.Ray.Start);
    const normal = isect.Thing.Normal(pos);
    const reflectDir = Vector_op_Subtraction_4C6022A0(d, Vector_op_Multiply_73181070(2 * Vector_Dot_4C6022A0(normal, d), normal));
    const naturalcolor = Color_op_Addition_Z1F82920(Color_get_DefaultColor(), RayTracer_GetNaturalColor(isect.Thing, pos, normal, reflectDir, scene));
    const reflectedColor = (depth >= RayTracer_maxDepth) ? Color_get_Grey() : RayTracer_GetReflectionColor(isect.Thing, Vector_op_Addition_4C6022A0(pos, Vector_op_Multiply_73181070(0.001, reflectDir)), normal, reflectDir, scene, depth);
    return Color_op_Addition_Z1F82920(naturalcolor, reflectedColor);
}

export function RayTracer_GetReflectionColor(thing, pos, normal, rd, scene, depth) {
    return Color_Scale_Z55C8D3EC(thing.Surface.Reflect(pos), RayTracer_TraceRay(new Ray(pos, rd), scene, depth + 1));
}

export function RayTracer_GetNaturalColor(thing, pos, normal, rd, scene) {
    let color = Color_get_DefaultColor();
    const arr = scene.Lights;
    for (let idx = 0; idx <= (arr.length - 1); idx++) {
        const light = arr[idx];
        color = RayTracer_AddLight(thing, pos, normal, rd, scene, color, light);
    }
    return color;
}

export function RayTracer_AddLight(thing, pos, normal, rd, scene, color, light) {
    const ldis = Vector_op_Subtraction_4C6022A0(light.Pos, pos);
    const livec = Vector_Norm_5620FEEB(ldis);
    const neatIsect = RayTracer_TestRay(new Ray(pos, livec), scene);
    let isInShadow;
    if (neatIsect != null) {
        const d = neatIsect;
        isInShadow = (!(d > Vector_Mag_5620FEEB(ldis)));
    }
    else {
        isInShadow = false;
    }
    if (isInShadow) {
        return color;
    }
    else {
        const illum = Vector_Dot_4C6022A0(livec, normal);
        const lcolor = (illum > 0) ? Color_Scale_Z55C8D3EC(illum, light.Color) : Color_get_DefaultColor();
        const specular = Vector_Dot_4C6022A0(livec, Vector_Norm_5620FEEB(rd));
        const scolor = (specular > 0) ? Color_Scale_Z55C8D3EC(Math.pow(specular, thing.Surface.Roughness), light.Color) : Color_get_DefaultColor();
        return Color_op_Addition_Z1F82920(Color_op_Addition_Z1F82920(color, Color_op_Multiply_Z1F82920(thing.Surface.Diffuse(pos), lcolor)), Color_op_Multiply_Z1F82920(thing.Surface.Specular(pos), scolor));
    }
}

export function RayTracer_GetPoint(x_1, y_1, width, height, camera) {
    const RecenterX = (x_2) => ((x_2 - (width / 2)) / (2 * width));
    const RecenterY = (y_2) => ((-(y_2 - (height / 2))) / (2 * height));
    return Vector_Norm_5620FEEB(Vector_op_Addition_4C6022A0(Vector_op_Addition_4C6022A0(Camera__get_Forward(camera), Vector_op_Multiply_73181070(RecenterX(x_1), Camera__get_Right(camera))), Vector_op_Multiply_73181070(RecenterY(y_1), Camera__get_Up(camera))));
}

export function RayTracer_Render(scene, data, x_1, y_1, width, height) {
    const clamp = (v) => {
        const value = min((x_3, y_3) => comparePrimitives(x_3, y_3), max((x_2, y_2) => comparePrimitives(x_2, y_2), v * 255, 0), 255);
        return value & 0xFF;
    };
    for (let y_4 = y_1; y_4 <= (height - 1); y_4++) {
        const stride = (y_4 * width) | 0;
        for (let x_4 = x_1; x_4 <= (width - 1); x_4++) {
            const index = ((x_4 + stride) * 4) | 0;
            const dir = RayTracer_GetPoint(x_4, y_4, width, height, scene.Camera);
            const ray = new Ray(Camera__get_Pos(scene.Camera), dir);
            const color = RayTracer_TraceRay(ray, scene, 0);
            data[index + 0] = clamp(color.R);
            data[index + 1] = clamp(color.G);
            data[index + 2] = clamp(color.B);
            data[index + 3] = 255;
        }
    }
}

export class SceneObjects_Sphere {
    constructor(center, radius, surface) {
        this.center = center;
        this.radius = radius;
        this.surface = surface;
    }
    get Surface() {
        const this$ = this;
        return this$.surface;
    }
    Normal(pos) {
        const this$ = this;
        return Vector_Norm_5620FEEB(Vector_op_Subtraction_4C6022A0(pos, this$.center));
    }
    Intersect(ray) {
        const this$ = this;
        const eo = Vector_op_Subtraction_4C6022A0(this$.center, ray.Start);
        const v = Vector_Dot_4C6022A0(eo, ray.Dir);
        let dist;
        if (v < 0) {
            dist = Number.POSITIVE_INFINITY;
        }
        else {
            const disc = (this$.radius * this$.radius) - (Vector_Dot_4C6022A0(eo, eo) - (v * v));
            dist = ((disc < 0) ? Number.POSITIVE_INFINITY : (v - Math.sqrt(disc)));
        }
        return dist;
    }
}

export function SceneObjects_Sphere$reflection() {
    return class_type("RayTracer.SceneObjects.Sphere", void 0, SceneObjects_Sphere);
}

export function SceneObjects_Sphere_$ctor_157D1BB7(center, radius, surface) {
    return new SceneObjects_Sphere(center, radius, surface);
}

export class SceneObjects_Plane {
    constructor(normal, offset, surface) {
        this.normal = normal;
        this.offset = offset;
        this.surface = surface;
    }
    get Surface() {
        const this$ = this;
        return this$.surface;
    }
    Normal(pos) {
        const this$ = this;
        return this$.normal;
    }
    Intersect(ray) {
        const this$ = this;
        const denom = Vector_Dot_4C6022A0(this$.normal, ray.Dir);
        const dist = (denom > 0) ? Number.POSITIVE_INFINITY : ((Vector_Dot_4C6022A0(this$.normal, ray.Start) + this$.offset) / (-denom));
        return dist;
    }
}

export function SceneObjects_Plane$reflection() {
    return class_type("RayTracer.SceneObjects.Plane", void 0, SceneObjects_Plane);
}

export function SceneObjects_Plane_$ctor_157D1BB7(normal, offset, surface) {
    return new SceneObjects_Plane(normal, offset, surface);
}

export class Surfaces_Shiny {
    constructor() {
    }
    Diffuse(pos) {
        return Color_get_White();
    }
    Specular(pos) {
        return Color_get_Grey();
    }
    Reflect(pos) {
        return 0.7;
    }
    get Roughness() {
        return 250;
    }
}

export function Surfaces_Shiny$reflection() {
    return class_type("RayTracer.Surfaces.Shiny", void 0, Surfaces_Shiny);
}

export function Surfaces_Shiny_$ctor() {
    return new Surfaces_Shiny();
}

export class Surfaces_Checkerboard {
    constructor() {
    }
    Diffuse(pos) {
        return (((~(~(Math.floor(pos.Z) + Math.floor(pos.X)))) % 2) !== 0) ? Color_get_White() : Color_get_Black();
    }
    Specular(pos) {
        return Color_get_White();
    }
    Reflect(pos) {
        return (((~(~(Math.floor(pos.Z) + Math.floor(pos.X)))) % 2) !== 0) ? 0.1 : 0.7;
    }
    get Roughness() {
        return 150;
    }
}

export function Surfaces_Checkerboard$reflection() {
    return class_type("RayTracer.Surfaces.Checkerboard", void 0, Surfaces_Checkerboard);
}

export function Surfaces_Checkerboard_$ctor() {
    return new Surfaces_Checkerboard();
}

export const Scenes_TwoSpheresOnACheckerboard = new Scene([SceneObjects_Plane_$ctor_157D1BB7(new Vector(0, 1, 0), 0, Surfaces_Checkerboard_$ctor()), SceneObjects_Sphere_$ctor_157D1BB7(new Vector(0, 1, -0.25), 1, Surfaces_Shiny_$ctor()), SceneObjects_Sphere_$ctor_157D1BB7(new Vector(-1, 0.5, 1.5), 0.5, Surfaces_Shiny_$ctor())], [new Light(new Vector(-2, 2.5, 0), new Color(0.49, 0.07, 0.07)), new Light(new Vector(1.5, 2.5, 1.5), new Color(0.07, 0.07, 0.49)), new Light(new Vector(1.5, 2.5, -1.5), new Color(0.07, 0.49, 0.071)), new Light(new Vector(0, 3.5, 0), new Color(0.21, 0.21, 0.35))], Camera_$ctor_4C6022A0(new Vector(3, 2, 4), new Vector(-1, 0.5, 0)));

export function renderScene(scene, x_1, y_1, width, height) {
    const canvas = document.getElementsByTagName("canvas")[0];
    const ctx = canvas.getContext('2d');
    const img = ctx.createImageData(width, height);
    RayTracer_Render(scene, img.data, x_1, y_1, width, height);
    ctx.putImageData(img, op_UnaryNegation_Int32(x_1), op_UnaryNegation_Int32(y_1));
}

export function measure(f, x_1, y_1) {
    const dtStart = window.performance.now();
    const res = f(x_1, y_1);
    const elapsed_1 = (window.performance.now()) - dtStart;
    return [res, elapsed_1];
}

const patternInput$0040249 = [0, 0, 1024, 1024];

export const y = patternInput$0040249[1];

export const x = patternInput$0040249[0];

export const w = patternInput$0040249[2];

export const h = patternInput$0040249[3];

const patternInput$0040250$002D1 = measure((scene, tupledArg) => {
    renderScene(scene, tupledArg[0], tupledArg[1], tupledArg[2], tupledArg[3]);
}, Scenes_TwoSpheresOnACheckerboard, [x, y, w, h]);

export const elapsed = patternInput$0040250$002D1[1];

toConsole(printf("Ray tracing:\n - rendered image size: (%dx%d)\n - elapsed: %f ms"))(w)(h)(elapsed);

